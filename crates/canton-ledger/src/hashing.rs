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
//! These are Canton's hashing schemes V2 and V3 (`HashingSchemeVersion.V2`,
//! `.V3`), byte for byte as `com.digitalasset.canton.protocol.hash` builds
//! them: every value is prefixed by a type tag, every variable-length item by
//! its length, every collection by its size, and the transaction and its
//! metadata are hashed separately before the final hash over both.
//!
//! V3 is V2 without the node and metadata encoding-version bytes, plus what
//! contract keys need: the key on create, fetch and exercise nodes, the
//! by-key flag on fetch and exercise, the lookup-by-key node, and the
//! metadata's maximum record time. A transaction that uses contract keys can
//! only be hashed under V3. The test module carries Canton's own vectors for
//! each node kind, the transaction, the metadata and the whole, under both.

use canton_proto::com::daml::ledger::api::v2 as api;
use canton_proto::com::daml::ledger::api::v2::interactive as ipb;
use canton_proto::com::daml::ledger::api::v2::interactive::transaction::v1 as node;
use sha2::{Digest as _, Sha256};
use std::collections::HashMap;

/// `HashPurpose.PreparedSubmission`.
const PURPOSE_PREPARED_SUBMISSION: u32 = 48;
/// The node encoding and the metadata encoding V2 prefixes; V3 dropped both.
const NODE_ENCODING_V1: u8 = 1;
const METADATA_ENCODING_V1: u8 = 1;
/// LF serialization versions as a prepared transaction spells them: `2.1`
/// for a node without a contract key, `2` for one with. V2 hashes only the
/// first, V3 both; anything newer (external call results) needs V4.
const SERIALIZATION_V1: &str = "2.1";
const SERIALIZATION_V2: &str = "2";
/// Deeper than any transaction a participant prepares; a hostile one could
/// nest exercises until the recursion below ran out of stack.
const MAX_DEPTH: usize = 256;

const CREATE_TAG: u8 = 0;
const EXERCISE_TAG: u8 = 1;
const FETCH_TAG: u8 = 2;
const ROLLBACK_TAG: u8 = 3;
const QUERY_BY_KEY_TAG: u8 = 4;

/// The hashing schemes this module implements.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Scheme {
    V2,
    V3,
}

impl Scheme {
    /// The scheme byte that follows the purpose in the final hash.
    fn byte(self) -> u8 {
        match self {
            Self::V2 => 2,
            Self::V3 => 3,
        }
    }

    fn name(self) -> &'static str {
        match self {
            Self::V2 => "V2",
            Self::V3 => "V3",
        }
    }

    /// Refuse an LF serialization version this scheme does not hash.
    fn check_version(self, lf_version: &str, what: &str) -> Result<(), HashingError> {
        let covered = match self {
            Self::V2 => lf_version == SERIALIZATION_V1,
            Self::V3 => lf_version == SERIALIZATION_V1 || lf_version == SERIALIZATION_V2,
        };
        if covered {
            return Ok(());
        }
        let hint = match self {
            Self::V2 if lf_version == SERIALIZATION_V2 => {
                "; it uses contract keys, so prepare it under V3"
            }
            _ => "",
        };
        Err(err(format!(
            "{what} uses LF serialization version `{lf_version}`, which hashing scheme {} does \
             not cover{hint}",
            self.name()
        )))
    }

    /// Refuse a contract key, or a by-key flag, the node may not carry: V2 has
    /// no encoding for either, and under V3 a node at serialization version
    /// `2.1` cannot have one (Canton refuses the same node the same way).
    fn check_key(self, lf_version: &str, keyed: bool, what: &str) -> Result<(), HashingError> {
        if !keyed {
            return Ok(());
        }
        match self {
            Self::V2 => Err(err(format!(
                "{what} uses a contract key, which hashing scheme V2 does not cover; prepare it \
                 under V3"
            ))),
            Self::V3 if lf_version == SERIALIZATION_V1 => Err(err(format!(
                "{what} uses a contract key but declares LF serialization version \
                 `{SERIALIZATION_V1}`, which has none"
            ))),
            Self::V3 => Ok(()),
        }
    }
}

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

    /// A node builder: no purpose; under V2 the node encoding version first.
    fn for_node(scheme: Scheme) -> Self {
        let mut builder = Self {
            digest: Sha256::new(),
        };
        if scheme == Scheme::V2 {
            builder.byte(NODE_ENCODING_V1);
        }
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

    /// A contract key with its maintainers (V3): the key's package name,
    /// template id, value and hash, then the maintainers as a set.
    fn key(&mut self, key: &ipb::GlobalKeyWithMaintainers) -> Result<&mut Self, HashingError> {
        let global = required(key.key.as_ref(), "a contract key without its key")?;
        let hash: Hash = global
            .hash
            .as_slice()
            .try_into()
            .map_err(|_| err("a contract key whose hash is not 32 bytes"))?;
        self.string(&global.package_name)?
            .identifier(required(
                global.template_id.as_ref(),
                "a contract key without a template id",
            )?)?
            .value(
                required(global.key.as_ref(), "a contract key without a value")?,
                0,
            )?
            .hash(&hash)
            .string_set(&key.maintainers)
    }

    fn optional_key(
        &mut self,
        key: Option<&ipb::GlobalKeyWithMaintainers>,
    ) -> Result<&mut Self, HashingError> {
        match key {
            Some(key) => {
                self.byte(1);
                self.key(key)
            }
            None => Ok(self.byte(0)),
        }
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

/// A create node, as a transaction node (with its seed) or as a disclosed
/// contract (without one).
fn hash_create(
    create: &node::Create,
    seed: Option<&Hash>,
    scheme: Scheme,
) -> Result<Hash, HashingError> {
    scheme.check_version(&create.lf_version, "a create node")?;
    scheme.check_key(&create.lf_version, create.key.is_some(), "a create node")?;
    let mut b = Builder::for_node(scheme);
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
    if scheme == Scheme::V3 {
        b.optional_key(create.key.as_ref())?;
    }
    Ok(b.finish())
}

fn hash_fetch(fetch: &node::Fetch, scheme: Scheme) -> Result<Hash, HashingError> {
    scheme.check_version(&fetch.lf_version, "a fetch node")?;
    scheme.check_key(
        &fetch.lf_version,
        fetch.key.is_some() || fetch.by_key,
        "a fetch node",
    )?;
    let mut b = Builder::for_node(scheme);
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
    if scheme == Scheme::V3 {
        b.bool(fetch.by_key).optional_key(fetch.key.as_ref())?;
    }
    Ok(b.finish())
}

/// A lookup-by-key node (V3 only): the key looked up and the contracts found.
fn hash_query_by_key(query: &node::QueryByKey, scheme: Scheme) -> Result<Hash, HashingError> {
    if scheme == Scheme::V2 {
        return Err(err(
            "a lookup-by-key node uses a contract key, which hashing scheme V2 does not cover; \
             prepare it under V3",
        ));
    }
    scheme.check_version(&query.lf_version, "a lookup-by-key node")?;
    scheme.check_key(&query.lf_version, true, "a lookup-by-key node")?;
    let mut b = Builder::for_node(scheme);
    b.string(&query.lf_version)?
        .byte(QUERY_BY_KEY_TAG)
        .string(&query.package_name)?
        .identifier(required(
            query.template_id.as_ref(),
            "a lookup-by-key node without a template id",
        )?)?
        .bool(query.exhaustive)
        .key(required(
            query.key.as_ref(),
            "a lookup-by-key node without a key",
        )?)?
        .count(query.result.len())?;
    for contract_id in &query.result {
        b.contract_id(contract_id)?;
    }
    Ok(b.finish())
}

struct Tree<'a> {
    nodes: HashMap<&'a str, &'a node::Node>,
    seeds: HashMap<i32, Hash>,
    scheme: Scheme,
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
                hash_create(create, Some(seed), self.scheme)
            }
            node::node::NodeType::Fetch(fetch) => hash_fetch(fetch, self.scheme),
            node::node::NodeType::Exercise(exercise) => {
                self.scheme
                    .check_version(&exercise.lf_version, "an exercise node")?;
                self.scheme.check_key(
                    &exercise.lf_version,
                    exercise.key.is_some() || exercise.by_key,
                    "an exercise node",
                )?;
                let seed = self
                    .seed(node_id)
                    .ok_or_else(|| err(format!("no node seed for exercise node `{node_id}`")))?;
                let mut b = Builder::for_node(self.scheme);
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
                b.string_set(&exercise.choice_observers)?;
                if self.scheme == Scheme::V3 {
                    b.bool(exercise.by_key)
                        .optional_key(exercise.key.as_ref())?;
                }
                b.count(exercise.children.len())?;
                for child in &exercise.children {
                    let child_hash = self.hash_node(child, depth + 1)?;
                    b.hash(&child_hash);
                }
                Ok(b.finish())
            }
            node::node::NodeType::Rollback(rollback) => {
                let mut b = Builder::for_node(self.scheme);
                b.byte(ROLLBACK_TAG).count(rollback.children.len())?;
                for child in &rollback.children {
                    let child_hash = self.hash_node(child, depth + 1)?;
                    b.hash(&child_hash);
                }
                Ok(b.finish())
            }
            node::node::NodeType::QueryByKey(query) => hash_query_by_key(query, self.scheme),
        }
    }
}

fn hash_transaction(
    transaction: &ipb::DamlTransaction,
    scheme: Scheme,
) -> Result<Hash, HashingError> {
    scheme.check_version(&transaction.version, "the transaction")?;
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
    let tree = Tree {
        nodes,
        seeds,
        scheme,
    };
    let mut b = Builder::with_purpose();
    b.string(&transaction.version)?
        .count(transaction.roots.len())?;
    for root in &transaction.roots {
        let root_hash = tree.hash_node(root, 0)?;
        b.hash(&root_hash);
    }
    Ok(b.finish())
}

fn hash_metadata(metadata: &ipb::Metadata, scheme: Scheme) -> Result<Hash, HashingError> {
    let submitter = required(
        metadata.submitter_info.as_ref(),
        "metadata without submitter info",
    )?;
    let mut b = Builder::with_purpose();
    if scheme == Scheme::V2 {
        b.byte(METADATA_ENCODING_V1);
    }
    b.strings(&submitter.act_as)?
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
        let contract_hash = hash_create(create, None, scheme)?;
        b.hash(&contract_hash);
    }
    if scheme == Scheme::V3 {
        match metadata.max_record_time {
            Some(micros) => {
                b.byte(1).unsigned_long(micros)?;
            }
            None => {
                b.byte(0);
            }
        }
    }
    Ok(b.finish())
}

fn hash_prepared(
    prepared: &ipb::PreparedTransaction,
    scheme: Scheme,
) -> Result<Hash, HashingError> {
    let transaction = required(
        prepared.transaction.as_ref(),
        "a prepared transaction without a transaction",
    )?;
    let metadata = required(
        prepared.metadata.as_ref(),
        "a prepared transaction without metadata",
    )?;
    let transaction_hash = hash_transaction(transaction, scheme)?;
    let metadata_hash = hash_metadata(metadata, scheme)?;
    let mut b = Builder::with_purpose();
    b.byte(scheme.byte())
        .hash(&transaction_hash)
        .hash(&metadata_hash);
    Ok(b.finish())
}

/// The hash the participant should have returned for `prepared` under
/// hashing scheme V2: what the signer signs.
///
/// # Errors
/// [`HashingError`] if the transaction is not something the V2 scheme
/// covers (a contract key, a node at a newer LF serialization version), or is
/// not self-consistent (a node missing, a seed missing, a contract id that is
/// not hex).
pub fn hash_prepared_transaction_v2(
    prepared: &ipb::PreparedTransaction,
) -> Result<[u8; 32], HashingError> {
    hash_prepared(prepared, Scheme::V2)
}

/// The hash the participant should have returned for `prepared` under
/// hashing scheme V3, the one a transaction with contract keys needs.
///
/// # Errors
/// As [`hash_prepared_transaction_v2`], for what V3 does not cover (a node at
/// a newer LF serialization version than keys need, such as external call
/// results).
pub fn hash_prepared_transaction_v3(
    prepared: &ipb::PreparedTransaction,
) -> Result<[u8; 32], HashingError> {
    hash_prepared(prepared, Scheme::V3)
}

/// The hash the participant should have returned for `prepared` under
/// `version`, for the schemes this module implements.
///
/// # Errors
/// [`HashingError`] for a scheme other than V2 or V3, and otherwise as
/// [`hash_prepared_transaction_v2`] and [`hash_prepared_transaction_v3`].
pub fn hash_prepared_transaction(
    prepared: &ipb::PreparedTransaction,
    version: ipb::HashingSchemeVersion,
) -> Result<[u8; 32], HashingError> {
    match version {
        ipb::HashingSchemeVersion::V2 => hash_prepared(prepared, Scheme::V2),
        ipb::HashingSchemeVersion::V3 => hash_prepared(prepared, Scheme::V3),
        other => Err(err(format!(
            "hashing scheme {other:?} is not one this SDK recomputes (it does V2 and V3)"
        ))),
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    //! Canton's own vectors (`protocol/hash/v2/` and `v3/`, `NodeHashTest.scala`
    //! and `MetadataHashTest.scala`, Canton 3.5.17), rebuilt as the API protos a
    //! participant returns.
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

    fn tree(tx: &ipb::DamlTransaction, scheme: Scheme) -> Tree<'_> {
        let mut nodes = HashMap::new();
        for entry in &tx.nodes {
            let Some(ipb::daml_transaction::node::VersionedNode::V1(node)) = &entry.versioned_node
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
        Tree {
            nodes,
            seeds,
            scheme,
        }
    }

    #[test]
    fn canton_vectors_for_each_node_kind() {
        let seed: Hash = hex(CREATE_SEED).try_into().unwrap();
        assert_eq!(
            to_hex(&hash_create(&create(), Some(&seed), Scheme::V2).unwrap()),
            "6d2cfe58c2294000592034f4bdfe397fe246901bb8b63e3b9e041bb478e174b7"
        );
        assert_eq!(
            to_hex(&hash_fetch(&fetch(), Scheme::V2).unwrap()),
            "c962c6098394f3d11cd6f0c795de9517d32a8e3e1979cec76cd2f66254efc610"
        );
        let tx = transaction();
        let tree = tree(&tx, Scheme::V2);
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
            to_hex(&hash_transaction(&transaction(), Scheme::V2).unwrap()),
            "154f334d24a8a5e4d0ce51ac87d93821b3256f885f21d3f779a1640abf481983"
        );
        assert_eq!(
            to_hex(&hash_metadata(&metadata(), Scheme::V2).unwrap()),
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
            hash_create(&create, None, Scheme::V2)
                .unwrap_err()
                .to_string()
                .contains("2.2")
        );

        let mut tx = transaction();
        tx.node_seeds.clear();
        assert!(
            hash_transaction(&tx, Scheme::V2)
                .unwrap_err()
                .to_string()
                .contains("seed")
        );

        let mut tx = transaction();
        tx.roots.push("9".to_string());
        assert!(
            hash_transaction(&tx, Scheme::V2)
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
            hash_transaction(&tx, Scheme::V2)
                .unwrap_err()
                .to_string()
                .contains("lookup-by-key")
        );

        let mut fetch = fetch();
        fetch.contract_id = "zz".to_string();
        assert!(
            hash_fetch(&fetch, Scheme::V2)
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
        let error = hash_transaction(&tx, Scheme::V2).unwrap_err();
        assert!(error.to_string().contains("deeper"), "{error}");
    }

    // V3: the same nodes as V2, at LF serialization version `2` and carrying a
    // contract key (and, on fetch and exercise, the by-key flag).
    const KEY_HASH: &str = "4d1741d27564ab1f1e74873034760583a1a71edaa90dbad6be3a78c0bf3eec7c";

    fn global_key() -> ipb::GlobalKeyWithMaintainers {
        ipb::GlobalKeyWithMaintainers {
            key: Some(ipb::GlobalKey {
                template_id: Some(identifier("package", "module_key", "name")),
                package_name: "package_name_key".to_string(),
                key: Some(text("hello")),
                hash: hex(KEY_HASH),
            }),
            maintainers: strings(&["david"]),
        }
    }

    fn create_v3() -> node::Create {
        node::Create {
            lf_version: "2".to_string(),
            key: Some(global_key()),
            ..create()
        }
    }

    fn fetch_v3() -> node::Fetch {
        node::Fetch {
            lf_version: "2".to_string(),
            key: Some(global_key()),
            by_key: true,
            ..fetch()
        }
    }

    fn exercise_v3(children: &[&str]) -> node::Exercise {
        node::Exercise {
            lf_version: "2".to_string(),
            key: Some(global_key()),
            by_key: true,
            ..exercise(children)
        }
    }

    fn query_v3() -> node::QueryByKey {
        node::QueryByKey {
            lf_version: "2".to_string(),
            package_name: "package-name-0".to_string(),
            template_id: Some(identifier("package", "module", "name")),
            exhaustive: true,
            key: Some(global_key()),
            result: strings(&[CID]),
        }
    }

    fn transaction_v3() -> ipb::DamlTransaction {
        let mut tx = transaction();
        tx.version = "2".to_string();
        tx.nodes = vec![
            wrap("0", node::node::NodeType::Create(create_v3())),
            wrap("1", node::node::NodeType::Fetch(fetch_v3())),
            wrap(
                "2",
                node::node::NodeType::Exercise(exercise_v3(&["0", "1"])),
            ),
            wrap(
                "3",
                node::node::NodeType::Rollback(node::Rollback {
                    children: strings(&["1", "2"]),
                }),
            ),
        ];
        tx
    }

    fn metadata_v3() -> ipb::Metadata {
        let mut metadata = metadata();
        for input in &mut metadata.input_contracts {
            if let Some(ipb::metadata::input_contract::Contract::V1(create)) = &mut input.contract {
                create.lf_version = "2".to_string();
            }
        }
        metadata.max_record_time = Some(2_592_000_000_000);
        metadata
    }

    #[test]
    fn canton_v3_vectors_for_each_node_kind() {
        let seed: Hash = hex(CREATE_SEED).try_into().unwrap();
        assert_eq!(
            to_hex(&hash_create(&create_v3(), Some(&seed), Scheme::V3).unwrap()),
            "0120d370509f54c07d8209f5af2d23f7f972997deb5fc5e0ebe886354395a3bc"
        );
        assert_eq!(
            to_hex(&hash_fetch(&fetch_v3(), Scheme::V3).unwrap()),
            "20437df1980bb7d4d94ad53181dc6005bae37b5990719070c48a3960e8dda3a4"
        );
        let tx = transaction_v3();
        let tree = tree(&tx, Scheme::V3);
        assert_eq!(
            to_hex(&tree.hash_node("2", 0).unwrap()),
            "516c4689acabf9d1f0087a02a7dafad884d9080c48112a57a6d20b9f72d4b697"
        );
        assert_eq!(
            to_hex(&tree.hash_node("3", 0).unwrap()),
            "ea61ab05f9ab9769ab5c91493a601ccad9e784bd4c0b3bc4b7777f0da68eab4d"
        );
        assert_eq!(
            to_hex(&hash_query_by_key(&query_v3(), Scheme::V3).unwrap()),
            "72c45c5a05a875621f16f2fe64d6c6db662ff41ee124d6c4f281409f5899b96c"
        );
    }

    #[test]
    fn canton_v3_vectors_for_the_transaction_the_metadata_and_the_whole() {
        // The two disclosed contracts, as the metadata trace hashes them.
        let metadata = metadata_v3();
        let disclosed: Vec<String> = metadata
            .input_contracts
            .iter()
            .map(|input| {
                let Some(ipb::metadata::input_contract::Contract::V1(create)) = &input.contract
                else {
                    unreachable!()
                };
                to_hex(&hash_create(create, None, Scheme::V3).unwrap())
            })
            .collect();
        assert_eq!(
            disclosed,
            [
                "84c7946905ba3afac9a2e8d360129b92c9055cc9792655f9a0c9195b08ade109",
                "e53bfab08a6142fc61781fcb242591824716501df665ebd4b68d7406c988f1b0"
            ]
        );
        assert_eq!(
            to_hex(&hash_transaction(&transaction_v3(), Scheme::V3).unwrap()),
            "36f0d1f4e9742a5cf54795fd1633f46b8235fafa93da006dafe10eb358d465c4"
        );
        assert_eq!(
            to_hex(&hash_metadata(&metadata, Scheme::V3).unwrap()),
            "5a4c6aa89bb80d0af05a3f0614ba07bc7469795fff66d7b9e87c876eb0137e0a"
        );
        let prepared = ipb::PreparedTransaction {
            transaction: Some(transaction_v3()),
            metadata: Some(metadata),
        };
        let full = "db37e4e251b83196260900fd43aacf934e1893b7ac58db4c1bfa8ced432b3b84";
        assert_eq!(
            to_hex(&hash_prepared_transaction_v3(&prepared).unwrap()),
            full
        );
        assert_eq!(
            to_hex(&hash_prepared_transaction(&prepared, ipb::HashingSchemeVersion::V3).unwrap()),
            full
        );
    }

    /// V3 hashes a transaction without keys too: V2's nodes at `2.1`, hashed
    /// without the encoding-version bytes, differ from their V2 hashes.
    #[test]
    fn v3_also_hashes_a_transaction_without_keys_and_not_as_v2_does() {
        let prepared = ipb::PreparedTransaction {
            transaction: Some(transaction()),
            metadata: Some(metadata()),
        };
        let v2 = hash_prepared_transaction_v2(&prepared).unwrap();
        let v3 = hash_prepared_transaction_v3(&prepared).unwrap();
        assert_ne!(v2, v3);
    }

    #[test]
    fn what_v2_and_v3_do_not_cover_is_refused_not_guessed() {
        // A keyed node under V2: refused, pointing at V3.
        let error = hash_create(&create_v3(), None, Scheme::V2).unwrap_err();
        assert!(error.to_string().contains("V3"), "{error}");
        let mut keyed_at_v1 = fetch();
        keyed_at_v1.by_key = true;
        let error = hash_fetch(&keyed_at_v1, Scheme::V2).unwrap_err();
        assert!(error.to_string().contains("V3"), "{error}");
        let error = hash_query_by_key(&query_v3(), Scheme::V2).unwrap_err();
        assert!(error.to_string().contains("V3"), "{error}");

        // A key on a node that says it has none (serialization `2.1`), under V3.
        let mut inconsistent = create_v3();
        inconsistent.lf_version = "2.1".to_string();
        let error = hash_create(&inconsistent, None, Scheme::V3).unwrap_err();
        assert!(error.to_string().contains("2.1"), "{error}");

        // Newer than V3 covers (external call results are `3`).
        let mut newer = create_v3();
        newer.lf_version = "3".to_string();
        assert!(hash_create(&newer, None, Scheme::V3).is_err());

        // A key whose hash is not a hash.
        let mut short = create_v3();
        short.key.as_mut().unwrap().key.as_mut().unwrap().hash = vec![1, 2, 3];
        let error = hash_create(&short, None, Scheme::V3).unwrap_err();
        assert!(error.to_string().contains("32 bytes"), "{error}");

        // A scheme this module does not implement.
        let prepared = ipb::PreparedTransaction {
            transaction: Some(transaction()),
            metadata: Some(metadata()),
        };
        assert!(hash_prepared_transaction(&prepared, ipb::HashingSchemeVersion::V4).is_err());
        assert!(
            hash_prepared_transaction(&prepared, ipb::HashingSchemeVersion::Unspecified).is_err()
        );
    }
}
