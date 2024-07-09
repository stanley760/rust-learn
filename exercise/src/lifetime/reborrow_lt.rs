#[allow(dead_code)]
struct Interface<'b, 'a: 'b> {
    manager: &'b mut Manager<'a>,
}

impl<'b, 'a: 'b> Interface<'b, 'a> {
    pub fn noop(self) {
        println!("interface consumed");
    }
}

struct Manager<'a> {
    name: &'a str,
}

struct List<'a> {
    manager: Manager<'a>,
}

impl<'a> List<'a> {
    pub fn get_interface<'b>(&'b mut self) -> Interface<'b, 'a>
    where
        'a: 'b,
    {
        Interface {
            manager: &mut self.manager,
        }
    }
}

pub fn invoke() {
    let mut list = List {
        manager: Manager { name: "manager" },
    };
    list.get_interface().noop();

    use_list(&list);
}

fn use_list(list: &List) {
    println!("list: {}", list.manager.name)
}
