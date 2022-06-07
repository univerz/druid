use std::{collections::HashMap, iter::repeat};

use itertools::Itertools;
use xilem::{list, scroll_view, App, AppLauncher, View};

fn app_logic(data: &mut AppData) -> impl View<AppData> {
    let data = unsafe { std::mem::transmute::<&mut AppData, &'static mut AppData>(data) };

    scroll_view(list(data.panel.list.len(), 16.0, move |idx| {
        let flat_item = &data.panel.list[idx];
        let item = data.tree.get(&flat_item.id).unwrap();
        format!(
            "{} * {}",
            repeat("    ").take(flat_item.depth).join(""),
            item.label
        )
    }))
}

fn main() {
    let app = App::new(AppData::new(), app_logic);
    AppLauncher::new(app).run();
}

pub struct Item {
    id: usize,
    label: String,
    children: Vec<usize>,
}

pub struct FlatItem {
    id: usize,
    depth: usize,
}

pub struct Panel {
    root: usize,
    list: Vec<FlatItem>,
}

struct AppData {
    tree: HashMap<usize, Item>,

    panel: Panel,
}

impl AppData {
    pub fn new() -> AppData {
        fn gen_children(path: &mut Vec<usize>, depth: usize, data: &mut AppData) {
            if let Some(cnt) = [100, 3, 2, 1].get(depth) {
                let parent_id = *path.last().unwrap();
                for _ in 0..*cnt {
                    let id = data.tree.len();
                    data.tree.get_mut(&parent_id).unwrap().children.push(id);
                    path.push(id);
                    data.tree.insert(
                        id,
                        Item {
                            id,
                            label: path.iter().map(ToString::to_string).join("-"),
                            children: Vec::new(),
                        },
                    );
                    gen_children(path, depth + 1, data);
                    path.pop();
                }
            }
        }
        let mut data = AppData {
            tree: vec![(
                0,
                Item {
                    id: 0,
                    label: "root".to_owned(),
                    children: Vec::new(),
                },
            )]
            .into_iter()
            .collect(),
            panel: Panel {
                root: 0,
                list: Vec::new(),
            },
        };
        gen_children(&mut vec![0], 0, &mut data);

        data.panel.list = data.flatten(data.panel.root);

        data
    }

    pub fn flatten(&mut self, root: usize) -> Vec<FlatItem> {
        fn rec(parent: usize, depth: usize, data: &AppData, list: &mut Vec<FlatItem>) {
            for &id in data.tree.get(&parent).unwrap().children.iter() {
                list.push(FlatItem { id, depth });
                rec(id, depth + 1, data, list);
            }
        }

        let mut list = Vec::new();
        rec(root, 0, self, &mut list);
        list
    }
}
