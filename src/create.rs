use std::path::Path;

pub enum CreateOperation {
    Schema,
    Event,
    Migration,
}

pub fn main(name: String, operation: CreateOperation) {
    let dir_name = match operation {
        CreateOperation::Schema => "schemas",
        CreateOperation::Event => "events",
        CreateOperation::Migration => "migrations",
    };

    // check that directory exists
    let folder_path = Path::new(dir_name);

    if !folder_path.exists() {
        panic!("Directory {} doesn't exist", dir_name);
    }

    let filename = match operation {
        CreateOperation::Schema => format!("{}.surql", name),
        CreateOperation::Event => format!("{}.surql", name),
        CreateOperation::Migration => {
            let now = chrono::Local::now();
            format!(
                "{}_{}_{}.surql",
                now.format("%Y%m%d"),
                now.format("%H%M%S"),
                name
            )
        }
    };

    // check that file doesn't already exist
    let file_path = folder_path.join(&filename);

    if file_path.exists() {
        panic!("File {} already exists", filename);
    }

    // generate content
    let content = match operation {
        CreateOperation::Schema => format!("DEFINE TABLE {} SCHEMALESS;", name),
        CreateOperation::Event => format!(
            "DEFINE TABLE {0} SCHEMALESS;

DEFINE EVENT {0} ON TABLE {0} WHEN $before == NONE THEN (
    # TODO
);",
            name
        ),
        CreateOperation::Migration => "".to_string(),
    };

    // create file
    fs_extra::file::write_all(&file_path, &content).unwrap();
}
