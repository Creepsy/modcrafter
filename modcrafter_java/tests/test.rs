fn main() {
    let test = modcrafter_java::JavaClass::new(
        "com.creepsy.main",
        "TestClass",
        Some("TestSuperClass"),
        vec!["Baum1", "Baum2", "Baum3"],
        vec![
            modcrafter_java::JavaField::new(
                "test_var",
                "0.25",
                modcrafter_java::JavaType::Float,
                modcrafter_java::JavaAccessLevel::Public,
            ),
            modcrafter_java::JavaField::new(
                "test_var2",
                "true",
                modcrafter_java::JavaType::Bool,
                modcrafter_java::JavaAccessLevel::Protected,
            ),
        ],
        Vec::new(),
    );
    println!("{}", test);
}
