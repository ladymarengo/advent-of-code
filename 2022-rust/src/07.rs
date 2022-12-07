use std::collections::HashMap;
use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input/07.txt").unwrap();

    let mut filesystem: HashMap<String, Item> = HashMap::new();
    let mut current = "/".to_string();
    filesystem.insert(
        current.clone(),
        Item {
            item_type: ItemType::Directory,
            size: 0,
            children: Vec::new(),
            parent: "".to_string(),
        },
    );

    input.lines().for_each(|line| {
        if line[..4].eq("$ cd") {
            if line[5..].eq("..") {
                current = filesystem.get(&current).unwrap().parent.to_string();
            } else if line[5..].eq("/") {
                current = "/".to_string();
            } else {
                current = format!("{}/{}", current, &line[5..]);
            }
        } else if line[..3].eq("dir") {
            let name = format!("{}/{}", current, &line[4..]);
            filesystem.insert(
                name.clone(),
                Item {
                    item_type: ItemType::Directory,
                    size: 0,
                    children: Vec::new(),
                    parent: current.clone(),
                },
            );
            let dir = filesystem.entry(current.clone()).or_insert(Item {
                item_type: ItemType::Directory,
                size: 0,
                children: Vec::new(),
                parent: current.clone(),
            });
            dir.children.push(name);
        } else if !line[..1].eq("$") {
            let (size, name) = line.split_once(' ').unwrap();
            let name = format!("{}/{}", current.clone(), name);
            filesystem.insert(
                name.clone(),
                Item {
                    item_type: ItemType::File,
                    size: size.parse::<usize>().unwrap(),
                    children: Vec::new(),
                    parent: current.clone(),
                },
            );
            let dir = filesystem.entry(current.clone()).or_insert(Item {
                item_type: ItemType::Directory,
                size: 0,
                children: Vec::new(),
                parent: current.clone(),
            });
            dir.children.push(name);
            let mut parent = current.clone();
            while !parent.eq("") {
                filesystem
                    .entry(parent.clone())
                    .or_insert(Item {
                        item_type: ItemType::Directory,
                        size: 0,
                        children: Vec::new(),
                        parent: "".to_string(),
                    })
                    .size += size.parse::<usize>().unwrap();
                parent = filesystem.get(&parent).unwrap().parent.clone();
            }
        }
    });

    println!(
        "{}",
        filesystem
            .values()
            .map(
                |d| if d.item_type == ItemType::Directory && d.size <= 100000 {
                    d.size
                } else {
                    0
                }
            )
            .sum::<usize>()
    );

    let delete = 30000000 - (70000000 - filesystem.get("/").unwrap().size);
    let mut possible_to_delete = filesystem
        .values()
        .filter_map(|v| {
            if v.item_type == ItemType::Directory && v.size >= delete {
                Some(v.size)
            } else {
                None
            }
        })
        .collect::<Vec<usize>>();
    possible_to_delete.sort();
    println!("{}", possible_to_delete.first().unwrap());
}

struct Item {
    item_type: ItemType,
    size: usize,
    children: Vec<String>,
    parent: String,
}

#[derive(Debug, PartialEq)]
enum ItemType {
    File,
    Directory,
}
