def parse_line(line):
    (rename, doc) = line.split("|", 1)
    rust_name = rename.replace(".", " ").replace("_", " ").replace("-", " ").replace(":", " ").title().replace(" ", "").strip()

    return (rust_name, rename, doc)



with open("message-type.txt", "r") as f:
    for line in f:
        (rust_name, serde_rename, docs) = parse_line(line)
        docs = docs.strip() if docs else "TODO: write this"
        print(f"/// {docs}")
        print(f"#[serde(rename = \"{serde_rename}\")]")
        print(f"{rust_name},")
        # break

