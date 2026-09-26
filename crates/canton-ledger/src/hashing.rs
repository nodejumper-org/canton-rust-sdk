//! The hash a prepared transaction is signed over, recomputed.
//!
//! Interactive submission hands the participant a command and gets back the
//! transaction it prepared and a hash of it; the signer signs the hash. Which
//! makes the hash the whole security of the scheme: a participant that returns
//! the caller's transaction beside the hash of a *different* one gets a valid
//! signature over the other one. So the hash is not taken from the response,
//! it is recomputed here from the transaction the caller can read, and the
//! response is refused if the two differ.
//!
//! This is Canton's hashing scheme V2 (`HashingSchemeVersion.V2`), byte for
//! byte as `com.digitalasset.canton.protocol.hash` builds it: every value is
//! prefixed by a type tag, every variable-length item by its length, every
//! collection by its size, and the transaction and its metadata are hashed
//! separately before the final hash over both. The test module carries
//! Canton's own vectors for each node kind, the transaction, the metadata and
//! the whole.

use canton_proto::com::daml::ledger::api::v2 as api;
use canton_proto::com::daml::ledger::api::v2::interactive as ipb;
use canton_proto::com::daml::ledger::api::v2::interactive::transaction::v1 as node;
use sha2::{Digest as _, Sha256};
use std::collections::HashMap;

/// `HashPurpose.PreparedSubmission`.
const PURPOSE_PREPARED_SUBMISSION: u32 = 48;
/// The scheme byte that follows the purpose in the final hash.
const HASHING_SCHEME_V2: u8 = 2;
/// The one node encoding and the one metadata encoding the V2 scheme has.
const NODE_ENCODING_V1: u8 = 1;
const METADATA_ENCODING_V1: u8 = 1;
/// The only LF serialization version the V2 scheme hashes. Anything newer
/// (contract keys, external call results) needs V3 or V4.
const SERIALIZATION_V1: &str = "2.1";
/// Deeper than any transaction a participant prepares; a hostile one could
/// nest exercises until the recursion below ran out of stack.
const MAX_DEPTH: usize = 256;

const CREATE_TAG: u8 = 0;
const EXERCISE_TAG: u8 = 1;
const FETCH_TAG: u8 = 2;
const ROLLBACK_TAG: u8 = 3;

/// Why a prepared transaction could not be hashed. Every case is either a
/// response the participant should not have sent or a feature the V2 scheme
/// does not cover; both mean the hash cannot be checked and the transaction
/// must not be signed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HashingError(String);

impl std::fmt::Display for HashingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for HashingError {}

fn err(message: impl Into<String>) -> HashingError {
    HashingError(message.into())
}

type Hash = [u8; 32];

/// Canton's `HashBuilder`: a SHA-256 fed the deterministic encoding.
struct Builder {
    digest: Sha256,
}

impl Builder {
    fn with_purpose() -> Self {
        let mut digest = Sha256::new();
        digest.update(PURPOSE_PREPARED_SUBMISSION.to_be_bytes());
        Self { digest }
    }

    /// A node builder: no purpose, the node encoding version first.
    fn for_node() -> Self {
        let mut builder = Self {
            digest: Sha256::new(),
        };
        builder.byte(NODE_ENCODING_V1);
        builder
    }

    fn byte(&mut self, value: u8) -> &mut Self {
        self.digest.update([value]);
        self
    }

    fn int(&mut self, value: i32) -> &mut Self {
        self.digest.update(value.to_be_bytes());
        self
    }

    fn count(&mut self, value: usize) -> Result<&mut Self, HashingError> {
        let value = i32::try_from(value).map_err(|_| err("a collection too large to encode"))?;
        Ok(self.int(value))
    }

    fn long(&mut self, value: i64) -> &mut Self {
        self.digest.update(value.to_be_bytes());
        self
    }

    fn unsigned_long(&mut self, value: u64) -> Result<&mut Self, HashingError> {
        let value = i64::try_from(value).map_err(|_| err("a time beyond what can be encoded"))?;
        Ok(self.long(value))
    }

    fn bool(&mut self, value: bool) -> &mut Self {
        self.byte(u8::from(value))
    }

    /// Length-prefixed bytes (`addByteString`).
    fn bytes(&mut self, value: &[u8]) -> Result<&mut Self, HashingError> {
        self.count(value.len())?;
        self.digest.update(value);
        Ok(self)
    }

    fn string(&mut self, value: &str) -> Result<&mut Self, HashingError> {
        self.bytes(value.as_bytes())
    }

    /// A hash, raw: fixed size, so no length (`addHash`, `addLfHash`).
    fn hash(&mut self, value: &Hash) -> &mut Self {
        self.digest.update(value);
        self
    }

    fn strings(&mut self, values: &[String]) -> Result<&mut Self, HashingError> {
        self.count(values.len())?;
        for value in values {
            self.string(value)?;
        }
        Ok(self)
    }

    /// `addStringSet`: sorted, unique, counted.
    fn string_set(&mut self, values: &[String]) -> Result<&mut Self, HashingError> {
        let mut sorted: Vec<&String> = values.iter().collect();
        sorted.sort();
        sorted.dedup();
        self.count(sorted.len())?;
        for value in sorted {
            self.string(value)?;
        }
        Ok(self)
    }

    fn dotted(&mut self, name: &str) -> Result<&mut Self, HashingError> {
        let segments: Vec<&str> = name.split('.').collect();
        self.count(segments.len())?;
        for segment in segments {
            self.string(segment)?;
        }
        Ok(self)
    }

    fn identifier(&mut self, id: &api::Identifier) -> Result<&mut Self, HashingError> {
        self.string(&id.package_id)?;
        self.dotted(&id.module_name)?;
        self.dotted(&id.entity_name)
    }

    fn optional_identifier(
        &mut self,
        id: Option<&api::Identifier>,
    ) -> Result<&mut Self, HashingError> {
        match id {
            Some(id) => {
                self.byte(1);
                self.identifier(id)
            }
            None => Ok(self.byte(0)),
        }
    }

    fn contract_id(&mut self, id: &str) -> Result<&mut Self, HashingError> {
        self.bytes(&decode_hex(id)?)
    }

    fn value(&mut self, value: &api::Value, depth: usize) -> Result<&mut Self, HashingError> {
        use api::value::Sum;
        if depth > MAX_DEPTH {
            return Err(err(
                "a value nested deeper than a prepared transaction can be",
            ));
        }
        let Some(sum) = &value.sum else {
            return Err(err("a value with nothing in it"));
        };
        match sum {
            Sum::Unit(()) => {
                self.byte(0);
            }
            Sum::Bool(b) => {
                self.byte(1).bool(*b);
            }
            Sum::Int64(i) => {
                self.byte(2).long(*i);
            }
            Sum::Numeric(n) => {
                self.byte(3).string(n)?;
            }
            Sum::Timestamp(t) => {
                self.byte(4).long(*t);
            }
            Sum::Date(d) => {
                self.byte(5).int(*d);
            }
            Sum::Party(p) => {
                self.byte(6).string(p)?;
            }
            Sum::Text(t) => {
                self.byte(7).string(t)?;
            }
            Sum::ContractId(id) => {
                self.byte(8).contract_id(id)?;
            }
            container => {
                self.container(container, depth)?;
            }
        }
        Ok(self)
    }

    /// The container values: each is a tag, a shape prefix, then its parts.
    fn container(
        &mut self,
        sum: &api::value::Sum,
        depth: usize,
    ) -> Result<&mut Self, HashingError> {
        use api::value::Sum;
        match sum {
            Sum::Optional(opt) => {
                self.byte(9);
                match &opt.value {
                    Some(inner) => {
                        self.byte(1).value(inner, depth + 1)?;
                    }
                    None => {
                        self.byte(0);
                    }
                }
            }
            Sum::List(list) => {
                self.byte(10).count(list.elements.len())?;
                for element in &list.elements {
                    self.value(element, depth + 1)?;
                }
            }
            Sum::TextMap(map) => {
                self.byte(11).count(map.entries.len())?;
                for entry in &map.entries {
                    self.string(&entry.key)?;
                    self.value(
                        required(entry.value.as_ref(), "a text-map entry without a value")?,
                        depth + 1,
                    )?;
                }
            }
            Sum::Record(record) => {
                self.byte(12)
                    .optional_identifier(record.record_id.as_ref())?
                    .count(record.fields.len())?;
                for field in &record.fields {
                    if field.label.is_empty() {
                        self.byte(0);
                    } else {
                        self.byte(1).string(&field.label)?;
                    }
                    self.value(
                        required(field.value.as_ref(), "a record field without a value")?,
                        depth + 1,
                    )?;
                }
            }
            Sum::Variant(variant) => {
                self.byte(13)
                    .optional_identifier(variant.variant_id.as_ref())?
                    .string(&variant.constructor)?
                    .value(
                        required(variant.value.as_ref(), "a variant without a value")?,
                        depth + 1,
                    )?;
            }
            Sum::Enum(e) => {
                self.byte(14)
                    .optional_identifier(e.enum_id.as_ref())?
                    .string(&e.constructor)?;
            }
            Sum::GenMap(map) => {
                self.byte(15).count(map.entries.len())?;
                for entry in &map.entries {
                    self.value(
                        required(entry.key.as_ref(), "a map entry without a key")?,
                        depth + 1,
                    )?;
                    self.value(
                        required(entry.value.as_ref(), "a map entry without a value")?,
                        depth + 1,
                    )?;
                }
            }
            _ => return Err(err("a scalar value in a container position")),
        }
        Ok(self)
    }

    fn finish(self) -> Hash {
        self.digest.finalize().into()
    }
}

fn required<'a, T>(value: Option<&'a T>, what: &str) -> Result<&'a T, HashingError> {
    value.ok_or_else(|| err(what))
}

fn decode_hex(text: &str) -> Result<Vec<u8>, HashingError> {
    if !text.len().is_multiple_of(2) {
        return Err(err(format!("`{text}` is not a contract id (odd length)")));
    }
    (0..text.len())
        .step_by(2)
        .map(|i| {
            u8::from_str_radix(&text[i..i + 2], 16)
                .map_err(|_| err(format!("`{text}` is not a contract id (not hex)")))
        })
        .collect()
}

fn version_v1(lf_version: &str, what: &str) -> Result<(), HashingError> {
    if lf_version == SERIALIZATION_V1 {
        Ok(())
    } else {
        Err(err(format!(
            "{what} uses LF serialization version `{lf_version}`, which hashing scheme V2 does not cover (only `{SERIALIZATION_V1}`); ask for a newer scheme"
        )))
    }
}

/// A create node, as a transaction node (with its seed) or as a disclosed
/// contract (without one).
fn hash_create(create: &node::Create, seed: Option<&Hash>) -> Result<Hash, HashingError> {
    version_v1(&create.lf_version, "a create node")?;
    if create.key.is_some() {
        return Err(err(
            "a create node with a contract key is not covered by hashing scheme V2",
        ));
    }
    let mut b = Builder::for_node();
    b.string(&create.lf_version)?.byte(CREATE_TAG);
    match seed {
        Some(seed) => {
            b.byte(1).hash(seed);
        }
        None => {
            b.byte(0);
        }
    }
    b.contract_id(&create.contract_id)?
        .string(&create.package_name)?
        .identifier(required(
            create.template_id.as_ref(),
            "a create node without a template id",
        )?)?
        .value(
            required(
                create.argument.as_ref(),
                "a create node without an argument",
            )?,
            0,
        )?
        .string_set(&create.signatories)?
        .string_set(&create.stakeholders)?;
    Ok(b.finish())
}

fn hash_fetch(fetch: &node::Fetch) -> Result<Hash, HashingError> {
    version_v1(&fetch.lf_version, "a fetch node")?;
    if fetch.key.is_some() || fetch.by_key {
        return Err(err(
            "a fetch node by key is not covered by hashing scheme V2",
        ));
    }
    let mut b = Builder::for_node();
    b.string(&fetch.lf_version)?
        .byte(FETCH_TAG)
        .contract_id(&fetch.contract_id)?
        .string(&fetch.package_name)?
        .identifier(required(
            fetch.template_id.as_ref(),
            "a fetch node without a template id",
        )?)?
        .string_set(&fetch.signatories)?
        .string_set(&fetch.stakeholders)?
        .optional_identifier(fetch.interface_id.as_ref())?
        .string_set(&fetch.acting_parties)?;
    Ok(b.finish())
}

struct Tree<'a> {
    nodes: HashMap<&'a str, &'a node::Node>,
    seeds: HashMap<i32, Hash>,
}

impl Tree<'_> {
    fn seed(&self, node_id: &str) -> Option<&Hash> {
        node_id
            .parse::<i32>()
            .ok()
            .and_then(|id| self.seeds.get(&id))
    }

    fn hash_node(&self, node_id: &str, depth: usize) -> Result<Hash, HashingError> {
        if depth > MAX_DEPTH {
            return Err(err(
                "a transaction nested deeper than a participant prepares",
            ));
        }
        let node = self.nodes.get(node_id).ok_or_else(|| {
            err(format!(
                "the transaction refers to node `{node_id}`, which it does not contain"
            ))
        })?;
        match node
            .node_type
            .as_ref()
            .ok_or_else(|| err(format!("node `{node_id}` is empty")))?
        {
            node::node::NodeType::Create(create) => {
                let seed = self
                    .seed(node_id)
                    .ok_or_else(|| err(format!("no node seed for create node `{node_id}`")))?;
                hash_create(create, Some(seed))
            }
            node::node::NodeType::Fetch(fetch) => hash_fetch(fetch),
            node::node::NodeType::Exercise(exercise) => {
                version_v1(&exercise.lf_version, "an exercise node")?;
                if exercise.key.is_some() || exercise.by_key {
                    return Err(err(
                        "an exercise node by key is not covered by hashing scheme V2",
                    ));
                }
                let seed = self
                    .seed(node_id)
                    .ok_or_else(|| err(format!("no node seed for exercise node `{node_id}`")))?;
                let mut b = Builder::for_node();
                b.string(&exercise.lf_version)?
                    .byte(EXERCISE_TAG)
                    .hash(seed)
                    .contract_id(&exercise.contract_id)?
                    .string(&exercise.package_name)?
                    .identifier(required(
                        exercise.template_id.as_ref(),
                        "an exercise node without a template id",
                    )?)?
                    .string_set(&exercise.signatories)?
                    .string_set(&exercise.stakeholders)?
                    .string_set(&exercise.acting_parties)?
                    .optional_identifier(exercise.interface_id.as_ref())?
                    .string(&exercise.choice_id)?
                    .value(
                        required(
                            exercise.chosen_value.as_ref(),
                            "an exercise node without a chosen value",
                        )?,
                        0,
                    )?
                    .bool(exercise.consuming);
                match &exercise.exercise_result {
                    Some(result) => {
                        b.byte(1).value(result, 0)?;
                    }
                    None => {
                        b.byte(0);
                    }
                }
                b.string_set(&exercise.choice_observers)?
                    .count(exercise.children.len())?;
                for child in &exercise.children {
                    let child_hash = self.hash_node(child, depth + 1)?;
                    b.hash(&child_hash);
                }
                Ok(b.finish())
            }
            node::node::NodeType::Rollback(rollback) => {
                let mut b = Builder::for_node();
                b.byte(ROLLBACK_TAG).count(rollback.children.len())?;
                for child in &rollback.children {
                    let child_hash = self.hash_node(child, depth + 1)?;
                    b.hash(&child_hash);
                }
                Ok(b.finish())
            }
            node::node::NodeType::QueryByKey(_) => Err(err(
                "a lookup-by-key node is not covered by hashing scheme V2",
            )),
        }
    }
}

fn hash_transaction(transaction: &ipb::DamlTransaction) -> Result<Hash, HashingError> {
    version_v1(&transaction.version, "the transaction")?;
    let mut nodes = HashMap::new();
    for entry in &transaction.nodes {
        let Some(ipb::daml_transaction::node::VersionedNode::V1(node)) = &entry.versioned_node
        else {
            return Err(err(format!("node `{}` is not a v1 node", entry.node_id)));
        };
        if nodes.insert(entry.node_id.as_str(), node).is_some() {
            return Err(err(format!("node id `{}` appears twice", entry.node_id)));
        }
    }
    let mut seeds = HashMap::new();
    for seed in &transaction.node_seeds {
        let bytes: Hash = seed
            .seed
            .as_slice()
            .try_into()
            .map_err(|_| err(format!("the seed of node {} is not 32 bytes", seed.node_id)))?;
        seeds.insert(seed.node_id, bytes);
    }
    let tree = Tree { nodes, seeds };
    let mut b = Builder::with_purpose();
    b.string(&transaction.version)?
        .count(transaction.roots.len())?;
    for root in &transaction.roots {
        let root_hash = tree.hash_node(root, 0)?;
        b.hash(&root_hash);
    }
    Ok(b.finish())
}

fn hash_metadata(metadata: &ipb::Metadata) -> Result<Hash, HashingError> {
    let submitter = required(
        metadata.submitter_info.as_ref(),
        "metadata without submitter info",
    )?;
    let mut b = Builder::with_purpose();
    b.byte(METADATA_ENCODING_V1)
        .strings(&submitter.act_as)?
        .string(&submitter.command_id)?
        .string(&metadata.transaction_uuid)?
        .int(
            i32::try_from(metadata.mediator_group)
                .map_err(|_| err("a mediator group beyond what can be encoded"))?,
        )
        .string(&metadata.synchronizer_id)?;
    for bound in [
        metadata.min_ledger_effective_time,
        metadata.max_ledger_effective_time,
    ] {
        match bound {
            Some(micros) => {
                b.byte(1).unsigned_long(micros)?;
            }
            None => {
                b.byte(0);
            }
        }
    }
    b.unsigned_long(metadata.preparation_time)?
        .count(metadata.input_contracts.len())?;
    for input in &metadata.input_contracts {
        let Some(ipb::metadata::input_contract::Contract::V1(create)) = &input.contract else {
            return Err(err("an input contract that is not a v1 create"));
        };
        b.unsigned_long(input.created_at)?;
        let contract_hash = hash_create(create, None)?;
        b.hash(&contract_hash);
    }
    Ok(b.finish())
}

/// The hash the participant should have returned for `prepared` under
/// hashing scheme V2: what the signer signs.
///
/// # Errors
/// [`HashingError`] if the transaction is not something the V2 scheme
/// covers, or is not self-consistent (a node missing, a seed missing, a
/// contract id that is not hex).
pub fn hash_prepared_transaction_v2(
    prepared: &ipb::PreparedTransaction,
) -> Result<[u8; 32], HashingError> {
    let transaction = required(
        prepared.transaction.as_ref(),
        "a prepared transaction without a transaction",
    )?;
    let metadata = required(
        prepared.metadata.as_ref(),
        "a prepared transaction without metadata",
    )?;
    let transaction_hash = hash_transaction(transaction)?;
    let metadata_hash = hash_metadata(metadata)?;
    let mut b = Builder::with_purpose();
    b.byte(HASHING_SCHEME_V2)
        .hash(&transaction_hash)
        .hash(&metadata_hash);
    Ok(b.finish())
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    //! Canton's own vectors (`protocol/hash/v2/NodeHashTest.scala` and
    //! `MetadataHashTest.scala`), rebuilt as the API protos a participant
    //! returns.
    use super::*;

    fn hex(text: &str) -> Vec<u8> {
        decode_hex(text).unwrap()
    }

    fn to_hex(hash: &Hash) -> String {
        use std::fmt::Write as _;
        hash.iter().fold(String::new(), |mut out, b| {
            let _ = write!(out, "{b:02x}");
            out
        })
    }

    fn identifier(package: &str, module: &str, name: &str) -> api::Identifier {
        api::Identifier {
            package_id: package.to_string(),
            module_name: module.to_string(),
            entity_name: name.to_string(),
        }
    }

    fn text(value: &str) -> api::Value {
        api::Value {
            sum: Some(api::value::Sum::Text(value.to_string())),
        }
    }

    fn strings(values: &[&str]) -> Vec<String> {
        values.iter().map(ToString::to_string).collect()
    }

    const CID: &str = "0007e7b5534931dfca8e1b485c105bae4e10808bd13ddc8e897f258015f9d921c5";
    const CREATE_SEED: &str = "926bbb6f341bc0092ae65d06c6e284024907148cc29543ef6bff0930f5d52c19";
    const EXERCISE_SEED: &str = "a867edafa1277f46f879ab92c373a15c2d75c5d86fec741705cee1eb01ef8c9e";

    fn create() -> node::Create {
        node::Create {
            lf_version: "2.1".to_string(),
            contract_id: CID.to_string(),
            package_name: "package-name-0".to_string(),
            template_id: Some(identifier("package", "module", "name")),
            argument: Some(text("hello")),
            signatories: strings(&["alice", "bob"]),
            stakeholders: strings(&["alice", "charlie"]),
            key: None,
        }
    }

    fn fetch() -> node::Fetch {
        node::Fetch {
            lf_version: "2.1".to_string(),
            contract_id: CID.to_string(),
            package_name: "package-name-0".to_string(),
            template_id: Some(identifier("package", "module", "name")),
            signatories: strings(&["alice"]),
            stakeholders: strings(&["charlie"]),
            acting_parties: strings(&["alice", "bob"]),
            interface_id: None,
            key: None,
            by_key: false,
        }
    }

    fn exercise(children: &[&str]) -> node::Exercise {
        node::Exercise {
            lf_version: "2.1".to_string(),
            contract_id: CID.to_string(),
            package_name: "package-name-0".to_string(),
            template_id: Some(identifier("package", "module", "name")),
            signatories: strings(&["alice"]),
            stakeholders: strings(&["charlie"]),
            acting_parties: strings(&["alice", "bob"]),
            interface_id: Some(identifier("package", "interface_module", "interface_name")),
            choice_id: "choice".to_string(),
            chosen_value: Some(api::Value {
                sum: Some(api::value::Sum::Int64(31380)),
            }),
            consuming: true,
            children: strings(children),
            exercise_result: Some(text("result")),
            choice_observers: strings(&["david"]),
            key: None,
            by_key: false,
        }
    }

    fn wrap(id: &str, node_type: node::node::NodeType) -> ipb::daml_transaction::Node {
        ipb::daml_transaction::Node {
            node_id: id.to_string(),
            versioned_node: Some(ipb::daml_transaction::node::VersionedNode::V1(node::Node {
                node_type: Some(node_type),
            })),
        }
    }

    /// Canton's transaction: roots `create` and `rollback`, the rollback holding
    /// `fetch` and `exercise`, the exercise holding `create` and `fetch` again.
    fn transaction() -> ipb::DamlTransaction {
        ipb::DamlTransaction {
            version: "2.1".to_string(),
            roots: strings(&["0", "3"]),
            nodes: vec![
                wrap("0", node::node::NodeType::Create(create())),
                wrap("1", node::node::NodeType::Fetch(fetch())),
                wrap("2", node::node::NodeType::Exercise(exercise(&["0", "1"]))),
                wrap(
                    "3",
                    node::node::NodeType::Rollback(node::Rollback {
                        children: strings(&["1", "2"]),
                    }),
                ),
            ],
            node_seeds: vec![
                ipb::daml_transaction::NodeSeed {
                    node_id: 0,
                    seed: hex(CREATE_SEED),
                },
                ipb::daml_transaction::NodeSeed {
                    node_id: 2,
                    seed: hex(EXERCISE_SEED),
                },
            ],
        }
    }

    fn disclosed(cid: &str, party: &str) -> node::Create {
        node::Create {
            lf_version: "2.1".to_string(),
            contract_id: cid.to_string(),
            package_name: "PkgName".to_string(),
            template_id: Some(identifier("-dummyPkg-", "DummyModule", "dummyName")),
            argument: Some(api::Value {
                sum: Some(api::value::Sum::ContractId(
                    "0097a092402108f5593bac7fb3c909cd316910197dd98d603042a45ab85c81e0fd"
                        .to_string(),
                )),
            }),
            signatories: strings(&[party]),
            stakeholders: strings(&[party]),
            key: None,
        }
    }

    fn metadata() -> ipb::Metadata {
        ipb::Metadata {
            submitter_info: Some(ipb::metadata::SubmitterInfo {
                act_as: strings(&["alice", "bob"]),
                command_id: "command-id".to_string(),
            }),
            synchronizer_id: "synchronizer::id".to_string(),
            mediator_group: 0,
            transaction_uuid: "4c6471d3-4e09-49dd-addf-6cd90e19c583".to_string(),
            preparation_time: 0,
            input_contracts: vec![
                ipb::metadata::InputContract {
                    contract: Some(ipb::metadata::input_contract::Contract::V1(disclosed(
                        CID, "alice",
                    ))),
                    created_at: 864_000_000_000,
                    event_blob: Vec::new(),
                },
                ipb::metadata::InputContract {
                    contract: Some(ipb::metadata::input_contract::Contract::V1(disclosed(
                        "0059b59ad7a6b6066e77b91ced54b8282f0e24e7089944685cb8f22f32fcbc4e1b",
                        "bob",
                    ))),
                    created_at: 1_728_000_000_000,
                    event_blob: Vec::new(),
                },
            ],
            min_ledger_effective_time: Some(43690),
            max_ledger_effective_time: Some(48059),
            ..Default::default()
        }
    }

    #[test]
    fn canton_vectors_for_each_node_kind() {
        let seed: Hash = hex(CREATE_SEED).try_into().unwrap();
        assert_eq!(
            to_hex(&hash_create(&create(), Some(&seed)).unwrap()),
            "6d2cfe58c2294000592034f4bdfe397fe246901bb8b63e3b9e041bb478e174b7"
        );
        assert_eq!(
            to_hex(&hash_fetch(&fetch()).unwrap()),
            "c962c6098394f3d11cd6f0c795de9517d32a8e3e1979cec76cd2f66254efc610"
        );
        let tx = transaction();
        let tree = {
            let mut nodes = HashMap::new();
            for entry in &tx.nodes {
                let Some(ipb::daml_transaction::node::VersionedNode::V1(node)) =
                    &entry.versioned_node
                else {
                    unreachable!()
                };
                nodes.insert(entry.node_id.as_str(), node);
            }
            let seeds = tx
                .node_seeds
                .iter()
                .map(|s| (s.node_id, s.seed.as_slice().try_into().unwrap()))
                .collect();
            Tree { nodes, seeds }
        };
        assert_eq!(
            to_hex(&tree.hash_node("2", 0).unwrap()),
            "070970eb4b2de72561dafb67017ca33850650a8103e5134e16044ba78991f48c"
        );
        assert_eq!(
            to_hex(&tree.hash_node("3", 0).unwrap()),
            "7264d5da2fd714427453bedc0d1cdb21f52ac7aec8d4bb5ac0598d25c5fcaed9"
        );
    }

    #[test]
    fn canton_vectors_for_the_transaction_the_metadata_and_the_whole() {
        assert_eq!(
            to_hex(&hash_transaction(&transaction()).unwrap()),
            "154f334d24a8a5e4d0ce51ac87d93821b3256f885f21d3f779a1640abf481983"
        );
        assert_eq!(
            to_hex(&hash_metadata(&metadata()).unwrap()),
            "6e89fcbcc9605179a47919b5e65a864e470e7a133f4f9f39b1e4545b223db769"
        );
        let prepared = ipb::PreparedTransaction {
            transaction: Some(transaction()),
            metadata: Some(metadata()),
        };
        assert_eq!(
            to_hex(&hash_prepared_transaction_v2(&prepared).unwrap()),
            "8c311c848db25d36b36fbd59f9483714a11688c2214e3c7cae3e028763520250"
        );
    }

    #[test]
    fn what_the_scheme_does_not_cover_is_refused_not_guessed() {
        let mut create = create();
        create.lf_version = "2.2".to_string();
        assert!(
            hash_create(&create, None)
                .unwrap_err()
                .to_string()
                .contains("2.2")
        );

        let mut tx = transaction();
        tx.node_seeds.clear();
        assert!(
            hash_transaction(&tx)
                .unwrap_err()
                .to_string()
                .contains("seed")
        );

        let mut tx = transaction();
        tx.roots.push("9".to_string());
        assert!(
            hash_transaction(&tx)
                .unwrap_err()
                .to_string()
                .contains("`9`")
        );

        let mut tx = transaction();
        tx.nodes.push(wrap(
            "4",
            node::node::NodeType::QueryByKey(node::QueryByKey::default()),
        ));
        tx.roots = strings(&["4"]);
        assert!(
            hash_transaction(&tx)
                .unwrap_err()
                .to_string()
                .contains("lookup-by-key")
        );

        let mut fetch = fetch();
        fetch.contract_id = "zz".to_string();
        assert!(
            hash_fetch(&fetch)
                .unwrap_err()
                .to_string()
                .contains("not hex")
        );
    }

    #[test]
    fn a_self_referencing_tree_is_cut_off_rather_than_recursed_into() {
        let mut tx = transaction();
        tx.nodes = vec![wrap("2", node::node::NodeType::Exercise(exercise(&["2"])))];
        tx.roots = strings(&["2"]);
        let error = hash_transaction(&tx).unwrap_err();
        assert!(error.to_string().contains("deeper"), "{error}");
    }
}
