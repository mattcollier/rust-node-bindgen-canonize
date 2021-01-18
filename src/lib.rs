extern crate rdf_canonize;

use node_bindgen::core::val::JsObject;
use node_bindgen::derive::node_bindgen;
use rdf_canonize::nquads::{
    Dataset, Graph, Object, Predicate, Quad, QuadSet, Subject, Term, TermType,
};

/// create array and fill with increase value
#[node_bindgen]
#[allow(unused_variables)]
fn canonize(quads: Vec<JsObject>, opts: JsObject) -> String {
    let q: QuadSet = quads.iter().map(|quad| parse_js_quad(quad)).collect();

    let dataset = Dataset {quads: q};

    rdf_canonize::canonize(&dataset, "URDNA2015").unwrap()
}

fn parse_js_quad(o: &JsObject) -> Quad {
    Quad {
        subject: parse_subject(o),
        predicate: parse_predicate(o),
        object: parse_object(o),
        graph: parse_graph(o),
    }
}

fn get_value(o: &JsObject, key: &str) -> String {
    get_string(o, key, &"value").unwrap()
}

fn get_term_type(o: &JsObject, key: &str) -> Option<TermType> {
    let term_str = get_string(o, key, &"termType").unwrap();
    match_term_type(&term_str)
}

fn get_datatype(o: &JsObject, key: &str) -> Option<String> {
    get_string(o, key, &"datatype")
}

fn get_language(o: &JsObject, key: &str) -> Option<String> {
    get_string(o, key, &"language")
}

fn match_term_type(t: &String) -> Option<TermType> {
    match t.as_str() {
        "BlankNode" => Some(TermType::BlankNode),
        "NamedNode" => Some(TermType::NamedNode),
        "Literal" => Some(TermType::Literal),
        "DefaultGraph" => Some(TermType::DefaultGraph),
        _ => None,
    }
}

fn parse_subject(o: &JsObject) -> Subject {
    let value = get_value(o, "subject");
    let term_type = get_term_type(o, "subject").unwrap();

    let mut subject = Subject::new();
    subject.set_term_type(&term_type);
    subject.set_value(&value);

    subject
}

fn parse_predicate(o: &JsObject) -> Predicate {
    let value = get_value(o, "predicate");
    let term_type = get_term_type(o, "predicate").unwrap();

    let mut predicate = Predicate::new();
    predicate.set_term_type(&term_type);
    predicate.set_value(&value);

    predicate
}

fn parse_object(o: &JsObject) -> Object {
    let value = get_value(o, "object");
    let term_type = get_term_type(o, "object").unwrap();
    match term_type {
        TermType::Literal => {
            let mut object = Object::new();
            object.set_term_type(&term_type);
            object.set_value(&value);

            // get datatype which always exists
            if let Some(datatype) = get_datatype(o, "object") {
                object.set_datatype(&datatype);
            }

            // get language which sometimes exists
            if let Some(language) = get_language(o, "object") {
                object.set_language(&language);
            }

            object
        }
        // BlankNode or NamedNode
        _ => {
            let mut object = Object::new();
            object.set_term_type(&term_type);
            object.set_value(&value);

            object
        }
    }
}

fn parse_graph(o: &JsObject) -> Graph {
    let value = get_value(o, "graph");
    let term_type = get_term_type(o, "graph").unwrap();
    let mut graph = Graph::new();
    graph.set_term_type(&term_type);
    graph.set_value(&value);

    graph
}

fn get_string(o: &JsObject, key1: &str, key2: &str) -> Option<String> {
    let s = o.get_property(key1).unwrap().unwrap();
    match s.get_property(key2).unwrap() {
        Some(v) => match key2 {
            "datatype" => {
                let d = v.get_property("value").unwrap().unwrap();
                let result = d.as_value::<String>();
                if let Err(_e) = result {
                    return None;
                }

                Some(result.unwrap())
            }
            _ => {
                let result = v.as_value::<String>();
                if let Err(_e) = result {
                    return None;
                }

                Some(result.unwrap())
            }
        },
        None => None,
    }
}
