use std::{
    collections::{HashMap, VecDeque},
    error::Error,
};

use crate::compiler::ast::{Doc, Type};

fn is_valid_type(
    t: &Type,
    include_docs: &Vec<&Doc>,
    adj_list: &mut HashMap<String, Vec<String>>,
    parrent_record: &str,
    dependency_map: &mut HashMap<String, String>,
) -> Result<(), Box<dyn Error>> {
    match t {
        Type::Boolean
        | Type::Byte
        | Type::Int
        | Type::Long
        | Type::Float
        | Type::Double
        | Type::UString
        | Type::Buffer => Ok(()),
        Type::Vector(inner_t) => is_valid_type(
            &inner_t,
            include_docs,
            adj_list,
            parrent_record,
            dependency_map,
        ),
        Type::Map(inner_t1, inner_t2) => {
            is_valid_type(
                inner_t1,
                include_docs,
                adj_list,
                parrent_record,
                dependency_map,
            )?;
            is_valid_type(
                &inner_t2,
                include_docs,
                adj_list,
                parrent_record,
                dependency_map,
            )?;
            Ok(())
        }
        Type::Class { name, namespace } => {
            if namespace.is_empty() {
                let mut match_count = 0;
                // in this case we will have to check for the matching classes with no collison,
                for doc in include_docs {
                    for module in &doc.modules {
                        for record in &module.records {
                            if &record.name == name {
                                adj_list
                                    .entry(parrent_record.to_string())
                                    .or_default()
                                    .push(format!("{}.{}", module.name, record.name));

                                dependency_map.insert(
                                    format!("{}.{}", parrent_record, name),
                                    format!("{}.{}", module.name, record.name),
                                );
                                match_count += 1;
                            }
                        }
                    }
                }
                if match_count == 0 {
                    return Err(format!("No match found for type {:?}", t).into());
                }
                if match_count == 1 {
                    return Ok(());
                } else {
                    return Err(format!("Ambigous name {:?}, multiple reference found", t).into());
                }
            } else {
                // we will have to search for fully qualifoed name
                for doc in include_docs {
                    for module in &doc.modules {
                        for record in &module.records {
                            if &record.name == name && &module.name == namespace {
                                adj_list
                                    .entry(parrent_record.to_string())
                                    .or_default()
                                    .push(format!("{}.{}", module.name, record.name));
                                dependency_map.insert(
                                    format!("{}.{}.{}", parrent_record, namespace, name),
                                    format!("{}.{}", module.name, record.name),
                                );

                                return Ok(());
                            }
                        }
                    }
                }
                return Err(format!("No match found for type {:?}", t).into());
            }
        }
    }
}

pub fn resolve_dependencies(docs: &Vec<Doc>) -> Result<HashMap<String, String>, Box<dyn Error>> {
    let mut src_to_docs = HashMap::new();
    for doc in docs {
        src_to_docs.insert(doc.src.clone(), doc);
    }
    // we will build a graph from type to type in this
    let mut adj_list = HashMap::new();
    let mut dependency_map = HashMap::new();
    for doc in docs {
        let mut included_docs: Vec<&Doc> = doc
            .includes
            .iter()
            .map(|included_src| {
                *src_to_docs.get(included_src).expect(&format!(
                    "Module {}, not found, included in {}",
                    included_src, doc.src
                ))
            })
            .collect();
        included_docs.push(doc);

        for module in &doc.modules {
            for record in &module.records {
                adj_list.insert(format!("{}.{}", module.name, record.name), Vec::new());
                for field in &record.fields {
                    if let Err(e) = is_valid_type(
                        &field._type,
                        &included_docs,
                        &mut adj_list,
                        &format!("{}.{}", module.name, record.name),
                        &mut dependency_map,
                    ) {
                        return Err(format!(
                            "Invalid Field {}, in record {}, in module {}, in file {}, error : {}",
                            field.name, record.name, module.name, doc.src, e
                        )
                        .into());
                    }
                }
            }
        }
    }
    // no we will have we have a edges going from one node to another now we will check if there is any references
    topological_sort(&adj_list)?;
    Ok(dependency_map)
}

fn topological_sort(adj_list: &HashMap<String, Vec<String>>) -> Result<(), Box<dyn Error>> {
    // we will check for all the edges
    let mut inorder = HashMap::new();
    for (key, value) in adj_list {
        for node in value {
            *inorder.entry(node).or_insert(0) += 1;
        }
    }
    let mut queue = VecDeque::new();
    for (key, _) in adj_list {
        if !inorder.contains_key(key) {
            queue.push_back(key);
        }
    }
    while !queue.is_empty() {
        let curr_node = queue.pop_front().unwrap();
        if !adj_list.contains_key(curr_node) {
            continue;
        }
        for adj in adj_list.get(curr_node).unwrap() {
            if let Some(curr_count) = inorder.get_mut(adj) {
                *curr_count -= 1;
                if *curr_count == 0 {
                    inorder.remove(adj);
                    queue.push_back(adj);
                }
            }
        }
    }
    if !inorder.is_empty() {
        return Err("Curcullar dependency found".into());
    }
    Ok(())
}
