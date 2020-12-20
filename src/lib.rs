use node_bindgen::derive::node_bindgen;
use node_bindgen::core::val::JsObject;

#[allow(non_camel_case_types)]
enum TermType {
    BLANK_NODE,
    NAMED_NODE,
    LITERAL,
    DEFAULT_GRAPH,
}

#[allow(dead_code)]
struct Term {
    term_type: TermType,
    value: String,
}

#[allow(dead_code)]
struct Quad {
    subject: Term,
    predicate: Term,
    object: Term,
    graph: Term,
}

struct Dataset {
    quad_set: Vec<Quad>,
}

impl Dataset {
    pub fn new() -> Dataset {
        Dataset {
            quad_set: vec![],
        }
    }
}

fn get_string(o: &JsObject, key1: &str, key2: &str) -> String {
    let s = o.get_property(key1).unwrap().unwrap();
    let v = s.get_property(key2).unwrap().unwrap();
    let y = v.as_value::<String>().unwrap();
    y
}

fn match_term_type(t: &String) -> Option<TermType> {
    match t.as_str() {
        "NamedNode" => Some(TermType::NAMED_NODE),
        "DefaultGraph" => Some(TermType::DEFAULT_GRAPH),
        "Literal" => Some(TermType::LITERAL),
        "BlankNode" => Some(TermType::BLANK_NODE),
        _ => None
    }
}

fn parse_term(o: &JsObject, key: &str) -> Term {
    let subject_value = get_string(o, key, &"value");
    let subject_term_type = get_string(o, key, &"termType");
    Term {
        term_type: match_term_type(&subject_term_type).unwrap(),
        value: subject_value,
    }
}

/// create array and fill with increase value
#[node_bindgen]
#[allow(unused_variables)]
fn canonize(quads: Vec<JsObject>, opts: JsObject) -> Vec<String> {
    // iterate the parameters
    let mut dataset = Dataset::new();
    for(_, q) in quads.iter().enumerate() {
        dataset.quad_set.push(Quad {
            subject: parse_term(q, "subject"),
            predicate: parse_term(q, "predicate"),
            object: parse_term(q, "object"),
            graph: parse_term(q, "graph"),
        });
    }

    let mut array = vec![];
    for (_, quad) in dataset.quad_set.iter().enumerate() {
        array.push(quad.subject.value.clone());
    }

    array
}
