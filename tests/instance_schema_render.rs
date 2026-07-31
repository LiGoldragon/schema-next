//! Rendering the per-instance schema trace through the schema encoder.
//!
//! Each test decodes a real value with the decoder-driven `DotosDecodeTraced`
//! (no hand walk), renders the captured trace with `InstanceSchemaText` (every
//! reference token through the schema encoder), asserts the rendered text
//! matches the endorsed form, and round-trips the rendered reference tokens
//! back through schema's `SourceReference::from_block`.

use dotos::{DotosDecodeTraced, DotosSource, InstanceSchema};
use schema::{InstanceSchemaText, SourceReference};

#[derive(Debug, PartialEq, Eq, dotos::DotosDecode, dotos::DotosDecodeTraced)]
enum Magnitude {
    Zero,
    Low,
    Medium,
    High,
}

#[derive(Debug, PartialEq, Eq, dotos::DotosDecode, dotos::DotosDecodeTraced)]
struct Certainty(Magnitude);

#[derive(Debug, PartialEq, Eq, dotos::DotosDecode, dotos::DotosDecodeTraced)]
struct Importance(Magnitude);

#[derive(Debug, PartialEq, Eq, dotos::DotosDecode, dotos::DotosDecodeTraced)]
struct Privacy(Magnitude);

#[derive(Debug, PartialEq, Eq, dotos::DotosDecode, dotos::DotosDecodeTraced)]
enum Kind {
    Decision,
    Principle,
    Constraint,
}

#[derive(Debug, PartialEq, Eq, dotos::DotosDecode, dotos::DotosDecodeTraced)]
struct Description(String);

#[derive(Debug, PartialEq, Eq, dotos::DotosDecode, dotos::DotosDecodeTraced)]
struct Referent(String);

#[derive(Debug, PartialEq, Eq, dotos::DotosDecode, dotos::DotosDecodeTraced)]
struct Referents(Vec<Referent>);

#[derive(Debug, PartialEq, Eq, dotos::DotosDecode, dotos::DotosDecodeTraced)]
enum Programming {
    CodeGeneration,
    Parsing,
}

#[derive(Debug, PartialEq, Eq, dotos::DotosDecode, dotos::DotosDecodeTraced)]
enum Software {
    Programming(Programming),
    Theory,
}

#[derive(Debug, PartialEq, Eq, dotos::DotosDecode, dotos::DotosDecodeTraced)]
enum Technology {
    Software(Software),
}

#[derive(Debug, PartialEq, Eq, dotos::DotosDecode, dotos::DotosDecodeTraced)]
enum Domain {
    Technology(Technology),
}

#[derive(Debug, PartialEq, Eq, dotos::DotosDecode, dotos::DotosDecodeTraced)]
struct Domains(Vec<Domain>);

#[derive(Debug, PartialEq, Eq, dotos::DotosDecode, dotos::DotosDecodeTraced)]
struct DomainScopes(Vec<Domain>);

#[derive(Debug, PartialEq, Eq, dotos::DotosDecode, dotos::DotosDecodeTraced)]
struct Partial(DomainScopes);

#[derive(Debug, PartialEq, Eq, dotos::DotosDecode, dotos::DotosDecodeTraced)]
struct Full(DomainScopes);

#[derive(Debug, PartialEq, Eq, dotos::DotosDecode, dotos::DotosDecodeTraced)]
enum DomainMatch {
    Any,
    Partial(Partial),
    Full(Full),
}

#[derive(Debug, PartialEq, Eq, dotos::DotosDecode, dotos::DotosDecodeTraced)]
struct Entry {
    domains: Domains,
    kind: Kind,
    description: Description,
    certainty: Certainty,
    importance: Importance,
    privacy: Privacy,
    referents: Referents,
}

#[derive(Debug, PartialEq, Eq, dotos::DotosDecode, dotos::DotosDecodeTraced)]
struct QuoteText(String);

#[derive(Debug, PartialEq, Eq, dotos::DotosDecode, dotos::DotosDecodeTraced)]
struct Antecedent(String);

#[derive(Debug, PartialEq, Eq, dotos::DotosDecode, dotos::DotosDecodeTraced)]
struct OptionalAntecedent(Option<Antecedent>);

#[derive(Debug, PartialEq, Eq, dotos::DotosDecode, dotos::DotosDecodeTraced)]
struct VerbatimQuote {
    quote_text: QuoteText,
    optional_antecedent: OptionalAntecedent,
}

#[derive(Debug, PartialEq, Eq, dotos::DotosDecode, dotos::DotosDecodeTraced)]
struct Testimony(Vec<VerbatimQuote>);

#[derive(Debug, PartialEq, Eq, dotos::DotosDecode, dotos::DotosDecodeTraced)]
struct Reasoning(String);

#[derive(Debug, PartialEq, Eq, dotos::DotosDecode, dotos::DotosDecodeTraced)]
struct Justification {
    testimony: Testimony,
    reasoning: Reasoning,
}

#[derive(Debug, PartialEq, Eq, dotos::DotosDecode, dotos::DotosDecodeTraced)]
struct RecordRequest {
    entry: Entry,
    justification: Justification,
}

#[derive(Debug, PartialEq, Eq, dotos::DotosDecode, dotos::DotosDecodeTraced)]
struct Record(RecordRequest);

#[derive(Debug, PartialEq, Eq, dotos::DotosDecode, dotos::DotosDecodeTraced)]
enum Input {
    Record(Record),
    Version,
}

fn schema_of<Value>(source: &str) -> InstanceSchema
where
    Value: DotosDecodeTraced,
{
    let block = DotosSource::new(source)
        .parse_root()
        .expect("parse a single root object");
    Value::from_dotos_block_traced(&block)
        .expect("decode value and capture its instance schema")
        .into_parts()
        .1
}

/// Every reference token the renderer emits must parse back through schema's
/// own reference reader.
fn round_trips_as_reference(text: &str) {
    let block = DotosSource::new(text)
        .parse_root()
        .expect("rendered reference parses as a DOTOS root");
    SourceReference::from_block(&block)
        .expect("rendered reference round-trips through SourceReference::from_block");
}

#[test]
fn enum_value_renders_the_enum_name() {
    let schema = schema_of::<Kind>("Decision");
    assert_eq!(InstanceSchemaText::new(&schema).aligned(), "Kind");
    assert_eq!(InstanceSchemaText::new(&schema).expanded(), "Kind");
}

#[test]
fn entry_renders_its_field_type_names() {
    let source = "{[Technology.Software.Programming.CodeGeneration] Decision (a description) High Medium Zero [spirit]}";
    let schema = schema_of::<Entry>(source);
    let rendered = InstanceSchemaText::new(&schema).aligned();
    assert_eq!(
        rendered,
        "{ Domains Kind Description Certainty Importance Privacy Referents }"
    );
}

#[test]
fn domain_match_partial_renders_enum_name_with_payload_reference() {
    let schema =
        schema_of::<DomainMatch>("Partial.[Technology.Software.Programming.CodeGeneration]");
    // The aligned enum payload collapses the transparent `Partial` wrapper to
    // its inner `DomainScopes` newtype name.
    let rendered = InstanceSchemaText::new(&schema).aligned();
    assert_eq!(rendered, "DomainMatch.DomainScopes");
    round_trips_as_reference("DomainMatch.DomainScopes");
}

#[test]
fn empty_domains_still_names_its_element_type() {
    let schema = schema_of::<Domains>("[]");
    // Aligned: the newtype wrapper name.
    assert_eq!(InstanceSchemaText::new(&schema).aligned(), "Domains");
    // Expanded: the newtype name plus the Vector.Domain container reference.
    assert_eq!(
        InstanceSchemaText::new(&schema).expanded(),
        "Domains.Vector.Domain"
    );
    round_trips_as_reference("Vector.Domain");
}

#[test]
fn root_input_record_renders_the_endorsed_root_form() {
    let source = "Record.{{[Technology.Software.Programming.CodeGeneration] Decision (a description) Medium Medium Zero [the spirit]} {[{(a quote) None}] (the reasoning)}}";
    let schema = schema_of::<Input>(source);
    // The endorsed one-to-one positional root form: enum name, the transparent
    // Record/RecordRequest wrappers collapsed, the payload a paren group of the
    // two aligned struct fields.
    let rendered = InstanceSchemaText::new(&schema).aligned();
    assert_eq!(
        rendered,
        "Input.({ Domains Kind Description Certainty Importance Privacy Referents } { Testimony Reasoning })"
    );
}

#[test]
fn certainty_newtype_renders_wrapper_then_magnitude() {
    let schema = schema_of::<Certainty>("High");
    assert_eq!(InstanceSchemaText::new(&schema).aligned(), "Certainty");
    assert_eq!(
        InstanceSchemaText::new(&schema).expanded(),
        "Certainty.Magnitude"
    );
    round_trips_as_reference("Certainty.Magnitude");
}
