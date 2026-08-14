//! Typed predicates, compiled to parameterized SQL.
//!
//! Nothing a caller supplies is ever spliced into the statement — not a value,
//! and not a JSON field path either. A path is sent as a `text[]` parameter and
//! applied with `#>`, so a field name taken from user input is data rather than
//! syntax. The only text this module concatenates is its own.
//!
//! What varies with the caller's *type*, not their input, is the qname: it is
//! `PACKAGE_NAME:MODULE_NAME:ENTITY_NAME`, read off the generated
//! [`Contract`](canton_daml::Contract), so a query names its template without
//! anybody typing a string.

use std::fmt::Write as _;

/// How a value is compared.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum Op {
    /// JSON equality — exact, and correct for every JSON type.
    Eq,
    /// JSON inequality.
    NotEq,
    /// Ordered comparison. Numbers compare numerically and strings compare as
    /// text; see [`Predicate::compare`].
    Lt,
    /// Ordered comparison.
    Le,
    /// Ordered comparison.
    Gt,
    /// Ordered comparison.
    Ge,
    /// JSONB containment (`@>`). The one that a GIN index on `payload` can
    /// serve, so it is worth reaching for on a large store.
    Contains,
}

impl Op {
    fn sql(self) -> &'static str {
        match self {
            Self::Eq => "=",
            Self::NotEq => "IS DISTINCT FROM",
            Self::Lt => "<",
            Self::Le => "<=",
            Self::Gt => ">",
            Self::Ge => ">=",
            Self::Contains => "@>",
        }
    }

    fn is_ordered(self) -> bool {
        matches!(self, Self::Lt | Self::Le | Self::Gt | Self::Ge)
    }
}

/// One condition on a contract's payload or on its row.
#[derive(Clone, Debug)]
pub struct Predicate {
    target: Target,
    op: Op,
    value: serde_json::Value,
}

#[derive(Clone, Debug)]
enum Target {
    /// A path into the JSONB payload.
    Payload(Vec<String>),
    /// The whole payload, for containment.
    WholePayload,
    /// A `text[]` column tested for membership.
    PartyList(&'static str),
    /// A plain text column.
    Text(&'static str),
}

impl Predicate {
    /// A field of the payload equals `value`.
    ///
    /// The comparison is JSON equality, so it is exact for every type: a
    /// `Numeric` that LF-JSON carries as `"10.0"` matches `"10.0"` and not
    /// `10.0`, which is the distinction the encoding makes on purpose.
    pub fn eq(path: impl IntoPath, value: impl Into<serde_json::Value>) -> Self {
        Self {
            target: Target::Payload(path.into_path()),
            op: Op::Eq,
            value: value.into(),
        }
    }

    /// A field of the payload does not equal `value` — **including when the
    /// field is absent**.
    ///
    /// Rendered as `IS DISTINCT FROM` rather than `<>`, and the difference is
    /// not pedantic here. `<>` against a missing path yields SQL `NULL`, so the
    /// row is excluded — and missing paths are the *normal* case for this
    /// crate, because [`qname`](Query::qname) deliberately matches on the
    /// package **name** so a query survives a Smart Contract Upgrade. A result
    /// set therefore mixes payloads of different versions with different field
    /// sets. Under `<>`, adding a field in v2 would silently drop every v1
    /// contract from a `not_eq` on it, with no error and no warning.
    pub fn not_eq(path: impl IntoPath, value: impl Into<serde_json::Value>) -> Self {
        Self {
            target: Target::Payload(path.into_path()),
            op: Op::NotEq,
            value: value.into(),
        }
    }

    /// An ordered comparison on a field.
    ///
    /// A contract whose payload lacks the field is **not** returned: there is
    /// no answer to "is the missing value greater than 10". That is the right
    /// verdict, unlike for [`not_eq`](Self::not_eq), but it is worth knowing
    /// when a query spans two versions of a template — use
    /// [`contains`](Self::contains) or a `not_eq` if the absent case should
    /// count.
    ///
    /// A JSON number compares numerically and a JSON string compares as text.
    /// That distinction is not cosmetic: LF-JSON carries `Int64` and `Numeric`
    /// as *strings*, so comparing an amount lexically would order `"9"` after
    /// `"10"`. Pass a number to compare numerically — the cast is applied for
    /// you — and a string only where the field really is text.
    pub fn compare(path: impl IntoPath, op: Op, value: impl Into<serde_json::Value>) -> Self {
        Self {
            target: Target::Payload(path.into_path()),
            op,
            value: value.into(),
        }
    }

    /// The payload contains this JSON object (`@>`).
    ///
    /// The predicate a GIN index on `payload` can serve.
    pub fn contains(value: impl Into<serde_json::Value>) -> Self {
        Self {
            target: Target::WholePayload,
            op: Op::Contains,
            value: value.into(),
        }
    }

    /// The party is among the contract's signatories.
    pub fn signatory(party: impl Into<String>) -> Self {
        Self::party_list("signatories", party)
    }

    /// The party is among the contract's observers.
    pub fn observer(party: impl Into<String>) -> Self {
        Self::party_list("observers", party)
    }

    /// The party is among the parties that witnessed it.
    pub fn witness(party: impl Into<String>) -> Self {
        Self::party_list("witnesses", party)
    }

    fn party_list(column: &'static str, party: impl Into<String>) -> Self {
        Self {
            target: Target::PartyList(column),
            op: Op::Eq,
            value: serde_json::Value::String(party.into()),
        }
    }

    /// The contract id is exactly this.
    pub fn contract_id(id: impl Into<String>) -> Self {
        Self {
            target: Target::Text("contract_id"),
            op: Op::Eq,
            value: serde_json::Value::String(id.into()),
        }
    }
}

/// Anything that names a path into the payload.
///
/// A single field is the common case; a slice reaches a nested one.
pub trait IntoPath {
    /// The path, one segment per level.
    fn into_path(self) -> Vec<String>;
}

impl IntoPath for &str {
    fn into_path(self) -> Vec<String> {
        vec![self.to_string()]
    }
}

impl IntoPath for String {
    fn into_path(self) -> Vec<String> {
        vec![self]
    }
}

impl<const N: usize> IntoPath for [&str; N] {
    fn into_path(self) -> Vec<String> {
        self.iter().map(ToString::to_string).collect()
    }
}

impl IntoPath for &[&str] {
    fn into_path(self) -> Vec<String> {
        self.iter().map(ToString::to_string).collect()
    }
}

impl IntoPath for Vec<String> {
    fn into_path(self) -> Vec<String> {
        self
    }
}

/// A parameter, in the form `tokio-postgres` will bind it.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum Param {
    /// A JSON value, bound as `jsonb`.
    Json(serde_json::Value),
    /// Text, bound as `text`.
    Text(String),
    /// A path, bound as `text[]`.
    Path(Vec<String>),
    /// A number, bound as `numeric` via its text form — Postgres parses it
    /// exactly, where a float would not.
    Numeric(String),
    /// A ledger offset.
    Offset(i64),
}

/// A compiled statement: SQL with `$n` placeholders, and the values for them.
#[derive(Clone, Debug, PartialEq)]
pub struct Sql {
    /// The statement.
    pub text: String,
    /// The parameters, in `$1..$n` order.
    pub params: Vec<Param>,
}

/// Which PQS function a query reads from.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum Source {
    /// Contracts active at an offset — the ACS.
    Active,
    /// Contracts created in a range.
    Creates,
    /// Contracts archived in a range.
    Archives,
}

impl Source {
    fn function(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Creates => "creates",
            Self::Archives => "archives",
        }
    }
}

/// A query over one template, compiled to parameterized SQL.
///
/// The template comes from the type, so the qname is never typed out:
///
/// ```rust,ignore
/// let sql = Query::<AppInstallRequest>::active()
///     .filter(Predicate::eq("user", party))
///     .limit(100)
///     .compile();
/// ```
#[derive(Clone, Debug)]
pub struct Query<T> {
    source: Source,
    predicates: Vec<Predicate>,
    limit: Option<i64>,
    offset_bounds: Option<(Option<i64>, Option<i64>)>,
    at_offset: Option<i64>,
    marker: std::marker::PhantomData<fn() -> T>,
}

impl<T: canton_daml::Contract> Query<T> {
    /// Contracts of this template that are active now.
    #[must_use]
    pub fn active() -> Self {
        Self::from_source(Source::Active, None, None)
    }

    /// Contracts of this template that were active **as of** an offset.
    ///
    /// What makes a paged or repeated read consistent: the ACS at a point,
    /// rather than a target that moves between calls.
    #[must_use]
    pub fn active_at(offset: i64) -> Self {
        Self::from_source(Source::Active, Some(offset), None)
    }

    /// Contracts of this template created between two offsets.
    ///
    /// `None` on either side keeps the store's own default — its oldest
    /// retained offset, and its latest.
    #[must_use]
    pub fn creates(from: Option<i64>, to: Option<i64>) -> Self {
        Self::from_source(Source::Creates, None, Some((from, to)))
    }

    /// Contracts of this template archived between two offsets.
    #[must_use]
    pub fn archives(from: Option<i64>, to: Option<i64>) -> Self {
        Self::from_source(Source::Archives, None, Some((from, to)))
    }

    fn from_source(
        source: Source,
        at_offset: Option<i64>,
        offset_bounds: Option<(Option<i64>, Option<i64>)>,
    ) -> Self {
        Self {
            source,
            predicates: Vec::new(),
            limit: None,
            offset_bounds,
            at_offset,
            marker: std::marker::PhantomData,
        }
    }

    /// The qname PQS knows this template by: `<package>:<Module>:<Entity>`.
    ///
    /// The package **name**, not its id — which is what makes a query survive
    /// a Smart Contract Upgrade instead of pinning one build.
    #[must_use]
    pub fn qname() -> String {
        format!("{}:{}:{}", T::PACKAGE_NAME, T::MODULE_NAME, T::ENTITY_NAME)
    }

    /// Add a condition. Conditions are combined with `AND`.
    #[must_use]
    pub fn filter(mut self, predicate: Predicate) -> Self {
        self.predicates.push(predicate);
        self
    }

    /// Cap the number of rows.
    #[must_use]
    pub fn limit(mut self, rows: i64) -> Self {
        self.limit = Some(rows);
        self
    }

    /// Compile to SQL and its parameters.
    ///
    /// Every caller-supplied value is a parameter — including JSON paths, which
    /// are bound as `text[]`. The statement text depends only on the *shape* of
    /// the query, never on its data.
    #[must_use]
    pub fn compile(&self) -> Sql {
        let mut params = vec![Param::Text(Self::qname())];
        let mut text = format!("SELECT * FROM {}($1", self.source.function());

        match (self.source, self.at_offset, self.offset_bounds) {
            (Source::Active, Some(offset), _) => {
                params.push(Param::Offset(offset));
                let _ = write!(text, ", ${}", params.len());
            }
            (Source::Creates | Source::Archives, _, Some((from, to))) => {
                // The two bounds are positional, so a `to` without a `from`
                // still has to pass something for the first: the function's own
                // default is spelled out rather than skipped.
                let from_sql = from.map_or_else(
                    || default_from(self.source).to_string(),
                    |value| {
                        params.push(Param::Offset(value));
                        format!("${}", params.len())
                    },
                );
                let to_sql = to.map_or_else(
                    || "latest_offset()".to_string(),
                    |value| {
                        params.push(Param::Offset(value));
                        format!("${}", params.len())
                    },
                );
                let _ = write!(text, ", {from_sql}, {to_sql}");
            }
            _ => {}
        }
        text.push(')');

        if !self.predicates.is_empty() {
            text.push_str(" WHERE ");
            let mut first = true;
            for predicate in &self.predicates {
                if !first {
                    text.push_str(" AND ");
                }
                first = false;
                render(predicate, &mut text, &mut params);
            }
        }

        if let Some(rows) = self.limit {
            params.push(Param::Offset(rows));
            let _ = write!(text, " LIMIT ${}", params.len());
        }

        Sql { text, params }
    }
}

/// The offset a range source starts from when the caller does not say.
fn default_from(source: Source) -> &'static str {
    match source {
        // `archives` starts at the pruning boundary where one exists, because
        // rows before it are gone; `creates` has no such boundary.
        Source::Archives => "COALESCE(pruned_offset(), oldest_offset())",
        _ => "oldest_offset()",
    }
}

fn render(predicate: &Predicate, text: &mut String, params: &mut Vec<Param>) {
    match &predicate.target {
        Target::Payload(path) => {
            params.push(Param::Path(path.clone()));
            let path_param = params.len();
            if predicate.op.is_ordered() {
                // `#>>` extracts text; the cast decides how it orders. LF-JSON
                // carries numbers as strings, so comparing an amount without
                // this would sort "9" after "10".
                match &predicate.value {
                    serde_json::Value::Number(n) => {
                        params.push(Param::Numeric(n.to_string()));
                        // `$n::text::numeric`, not `$n::numeric`: a bare cast
                        // makes Postgres infer the parameter as `numeric`, and
                        // the driver has no Rust type that binds to it. Bound
                        // as text and parsed by Postgres, a decimal keeps every
                        // digit — which a float would not.
                        let _ = write!(
                            text,
                            "(payload #>> ${path_param})::numeric {} ${}::text::numeric",
                            predicate.op.sql(),
                            params.len()
                        );
                    }
                    other => {
                        params.push(Param::Text(json_text(other)));
                        let _ = write!(
                            text,
                            "payload #>> ${path_param} {} ${}",
                            predicate.op.sql(),
                            params.len()
                        );
                    }
                }
            } else {
                params.push(Param::Json(predicate.value.clone()));
                let _ = write!(
                    text,
                    "payload #> ${path_param} {} ${}",
                    predicate.op.sql(),
                    params.len()
                );
            }
        }
        Target::WholePayload => {
            params.push(Param::Json(predicate.value.clone()));
            let _ = write!(text, "payload {} ${}", predicate.op.sql(), params.len());
        }
        Target::PartyList(column) => {
            params.push(Param::Text(json_text(&predicate.value)));
            let _ = write!(text, "${} = ANY({column})", params.len());
        }
        Target::Text(column) => {
            params.push(Param::Text(json_text(&predicate.value)));
            let _ = write!(text, "{column} {} ${}", predicate.op.sql(), params.len());
        }
    }
}

/// A JSON value as the text Postgres should compare against — a string without
/// its quotes, anything else as it is written.
fn json_text(value: &serde_json::Value) -> String {
    match value {
        serde_json::Value::String(s) => s.clone(),
        other => other.to_string(),
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;
    use canton_quickstart_licensing::quickstart_licensing::Licensing_AppInstall::AppInstallRequest;

    /// The qname is derived from the type, so nobody types it — and it uses the
    /// package *name*, which is what survives an upgrade.
    #[test]
    fn the_template_names_itself() {
        assert_eq!(
            Query::<AppInstallRequest>::qname(),
            "quickstart-licensing:Licensing.AppInstall:AppInstallRequest"
        );
    }

    /// The bare query reads the ACS for one template, with the qname as a
    /// parameter rather than spliced into the text.
    #[test]
    fn a_bare_query_is_one_function_call_with_one_parameter() {
        let sql = Query::<AppInstallRequest>::active().compile();
        assert_eq!(sql.text, "SELECT * FROM active($1)");
        assert_eq!(
            sql.params,
            vec![Param::Text(
                "quickstart-licensing:Licensing.AppInstall:AppInstallRequest".to_string()
            )]
        );
    }

    /// The property this module exists for: a field name from outside is a
    /// parameter, not syntax. Postgres never parses it, so there is nothing to
    /// inject into.
    #[test]
    fn a_hostile_field_name_is_data_and_not_syntax() {
        let sql = Query::<AppInstallRequest>::active()
            .filter(Predicate::eq("user'; DROP TABLE __contracts; --", "x"))
            .compile();

        assert_eq!(
            sql.text,
            "SELECT * FROM active($1) WHERE payload #> $2 = $3"
        );
        assert!(
            !sql.text.contains("DROP"),
            "the field name must not reach the statement: {}",
            sql.text
        );
        assert_eq!(
            sql.params[1],
            Param::Path(vec!["user'; DROP TABLE __contracts; --".to_string()])
        );
    }

    #[test]
    fn a_hostile_value_is_a_parameter_too() {
        let sql = Query::<AppInstallRequest>::active()
            .filter(Predicate::eq("user", "'; DROP TABLE __contracts; --"))
            .compile();
        assert!(!sql.text.contains("DROP"), "{}", sql.text);
        assert_eq!(
            sql.params[2],
            Param::Json(serde_json::json!("'; DROP TABLE __contracts; --"))
        );
    }

    /// `<>` against a missing JSON path yields SQL NULL, so the row is
    /// excluded. That matters here more than it would elsewhere: a query
    /// matches on the package **name** so it survives an upgrade, which means a
    /// result set mixes payloads of different versions with different field
    /// sets. Under `<>`, adding a field in v2 would silently drop every v1
    /// contract from a `not_eq` on it — no error, no warning, just a short
    /// answer.
    #[test]
    fn not_equal_still_matches_a_contract_whose_field_is_absent() {
        let sql = Query::<AppInstallRequest>::active()
            .filter(Predicate::not_eq("status", "closed"))
            .compile();
        assert!(
            sql.text.contains("IS DISTINCT FROM"),
            "not_eq must include the absent case: {}",
            sql.text
        );
        assert!(
            !sql.text.contains(" <> "),
            "plain `<>` drops rows lacking the field: {}",
            sql.text
        );
    }

    /// A nested path reaches into the payload without any string building.
    #[test]
    fn a_nested_path_is_one_array_parameter() {
        let sql = Query::<AppInstallRequest>::active()
            .filter(Predicate::eq(["meta", "values", "reason"], "renewal"))
            .compile();
        assert_eq!(
            sql.params[1],
            Param::Path(vec![
                "meta".to_string(),
                "values".to_string(),
                "reason".to_string()
            ])
        );
    }

    /// LF-JSON carries numbers as strings, so an ordered comparison on an
    /// amount must be numeric. Compared as text, "9" sorts after "10" and every
    /// threshold query is quietly wrong.
    #[test]
    fn an_ordered_comparison_on_a_number_is_numeric_and_not_lexical() {
        let sql = Query::<AppInstallRequest>::active()
            .filter(Predicate::compare("amount", Op::Gt, 100))
            .compile();
        assert_eq!(
            sql.text,
            "SELECT * FROM active($1) WHERE (payload #>> $2)::numeric > $3::text::numeric"
        );
        assert_eq!(sql.params[2], Param::Numeric("100".to_string()));
    }

    /// A comparison on text stays textual — casting it to numeric would fail
    /// at the database rather than compare wrongly, but it would still be the
    /// wrong statement.
    #[test]
    fn an_ordered_comparison_on_text_stays_textual() {
        let sql = Query::<AppInstallRequest>::active()
            .filter(Predicate::compare("name", Op::Ge, "m"))
            .compile();
        assert_eq!(
            sql.text,
            "SELECT * FROM active($1) WHERE payload #>> $2 >= $3"
        );
        assert_eq!(sql.params[2], Param::Text("m".to_string()));
    }

    /// Containment is the one an index can serve, so it is offered by name.
    #[test]
    fn containment_uses_the_operator_an_index_can_serve() {
        let sql = Query::<AppInstallRequest>::active()
            .filter(Predicate::contains(
                serde_json::json!({ "user": "alice::1220" }),
            ))
            .compile();
        assert_eq!(sql.text, "SELECT * FROM active($1) WHERE payload @> $2");
    }

    #[test]
    fn party_columns_are_tested_for_membership() {
        let sql = Query::<AppInstallRequest>::active()
            .filter(Predicate::signatory("alice::1220"))
            .filter(Predicate::observer("bob::1220"))
            .compile();
        assert_eq!(
            sql.text,
            "SELECT * FROM active($1) WHERE $2 = ANY(signatories) AND $3 = ANY(observers)"
        );
        assert_eq!(sql.params[1], Param::Text("alice::1220".to_string()));
    }

    #[test]
    fn an_offset_pins_the_acs_to_a_point_in_time() {
        let sql = Query::<AppInstallRequest>::active_at(42).compile();
        assert_eq!(sql.text, "SELECT * FROM active($1, $2)");
        assert_eq!(sql.params[1], Param::Offset(42));
    }

    /// The bounds are positional. A `to` without a `from` must still pass
    /// something for the first argument, and the function's own default is
    /// what it passes — not a guess.
    #[test]
    fn an_upper_bound_alone_still_names_the_lower_one() {
        let sql = Query::<AppInstallRequest>::creates(None, Some(99)).compile();
        assert_eq!(sql.text, "SELECT * FROM creates($1, oldest_offset(), $2)");
        assert_eq!(sql.params[1], Param::Offset(99));

        // And archives starts at the pruning boundary, because rows before it
        // are not there to be read.
        let sql = Query::<AppInstallRequest>::archives(None, Some(99)).compile();
        assert_eq!(
            sql.text,
            "SELECT * FROM archives($1, COALESCE(pruned_offset(), oldest_offset()), $2)"
        );
    }

    #[test]
    fn both_bounds_are_parameters_when_both_are_given() {
        let sql = Query::<AppInstallRequest>::creates(Some(10), Some(20)).compile();
        assert_eq!(sql.text, "SELECT * FROM creates($1, $2, $3)");
        assert_eq!(sql.params[1], Param::Offset(10));
        assert_eq!(sql.params[2], Param::Offset(20));
    }

    /// Placeholders have to be numbered in the order the parameters are bound,
    /// or every query after the first predicate reads the wrong value.
    #[test]
    fn placeholders_are_numbered_in_binding_order() {
        let sql = Query::<AppInstallRequest>::active()
            .filter(Predicate::eq("user", "alice"))
            .filter(Predicate::signatory("bob"))
            .filter(Predicate::compare("amount", Op::Lt, 5))
            .limit(10)
            .compile();

        assert_eq!(
            sql.text,
            "SELECT * FROM active($1) WHERE payload #> $2 = $3 AND $4 = ANY(signatories) \
             AND (payload #>> $5)::numeric < $6::text::numeric LIMIT $7"
        );
        assert_eq!(sql.params.len(), 7);
        assert_eq!(sql.params[6], Param::Offset(10));
    }
}
