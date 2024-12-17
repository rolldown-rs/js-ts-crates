use starbase_sandbox::create_empty_sandbox;
use typescript_tsconfig_json::{CompilerPath, ExtendsField, TsConfigExtendsChain, TsConfigJson};

#[test]
fn handles_path_types() {
    let tsconfig: TsConfigJson = serde_json::from_str(
        r#"{
  "include": ["file.ts", "dir/**/*"],
  "exclude": ["file.tsx?", "dir"],
  "files": ["file.tsx"]
}"#,
    )
    .unwrap();

    assert_eq!(
        tsconfig,
        TsConfigJson {
            include: Some(vec![
                CompilerPath::from("file.ts"),
                CompilerPath::from("dir/**/*"),
            ]),
            exclude: Some(vec![
                CompilerPath::from("file.tsx?"),
                CompilerPath::from("dir")
            ]),
            files: Some(vec![CompilerPath::from("file.tsx")]),
            ..Default::default()
        }
    );
}

mod node_modules_config {
    use super::*;

    #[test]
    fn returns_none_if_not_exists() {
        let sandbox = create_empty_sandbox();

        assert_eq!(
            TsConfigJson::resolve_path_in_node_modules(
                "@scope/package/tsconfig.json",
                sandbox.path()
            ),
            None
        );
    }

    #[test]
    fn resolves_with_package_name() {
        let sandbox = create_empty_sandbox();
        sandbox.create_file("node_modules/@scope/package/tsconfig.json", "{}");

        assert_eq!(
            TsConfigJson::resolve_path_in_node_modules("@scope/package", sandbox.path()),
            Some(
                sandbox
                    .path()
                    .join("node_modules/@scope/package/tsconfig.json")
            )
        );
    }

    #[test]
    fn resolves_with_package_name_and_custom_file() {
        let sandbox = create_empty_sandbox();
        sandbox.create_file("node_modules/@scope/package/tsconfig.alt.json", "{}");

        assert_eq!(
            TsConfigJson::resolve_path_in_node_modules(
                "@scope/package/tsconfig.json",
                sandbox.path()
            ),
            None
        );
        assert_eq!(
            TsConfigJson::resolve_path_in_node_modules(
                "@scope/package/tsconfig.alt.json",
                sandbox.path()
            ),
            Some(
                sandbox
                    .path()
                    .join("node_modules/@scope/package/tsconfig.alt.json")
            )
        );
    }

    #[test]
    fn resolves_nested_file_in_package() {
        let sandbox = create_empty_sandbox();
        sandbox.create_file("node_modules/@scope/package/configs/tsconfig.json", "{}");

        assert_eq!(
            TsConfigJson::resolve_path_in_node_modules(
                "@scope/package/configs/tsconfig.json",
                sandbox.path()
            ),
            Some(
                sandbox
                    .path()
                    .join("node_modules/@scope/package/configs/tsconfig.json")
            )
        );
        assert_eq!(
            TsConfigJson::resolve_path_in_node_modules("@scope/package/configs", sandbox.path()),
            Some(
                sandbox
                    .path()
                    .join("node_modules/@scope/package/configs/tsconfig.json")
            )
        );
    }

    #[test]
    fn resolves_traversing_upwards() {
        let sandbox = create_empty_sandbox();
        sandbox.create_file("node_modules/@scope/package/tsconfig.json", "{}");

        assert_eq!(
            TsConfigJson::resolve_path_in_node_modules(
                "@scope/package",
                sandbox.path().join("nested/project/with/node_modules")
            ),
            Some(
                sandbox
                    .path()
                    .join("node_modules/@scope/package/tsconfig.json")
            )
        );
    }
}

mod extends_chain {
    use super::*;

    #[test]
    fn resolves_self() {
        let sandbox = create_empty_sandbox();
        sandbox.create_file("tsconfig.json", "{}");

        let chain =
            TsConfigJson::resolve_extends_chain(sandbox.path().join("tsconfig.json")).unwrap();

        assert_eq!(
            chain,
            vec![TsConfigExtendsChain {
                path: sandbox.path().join("tsconfig.json"),
                config: TsConfigJson::default()
            }]
        );
    }

    #[test]
    fn resolves_single() {
        let sandbox = create_empty_sandbox();
        sandbox.create_file(
            "tsconfig.json",
            r#"{ "extends": "./tsconfig.1.json", "include": ["file.tsx"] }"#,
        );
        sandbox.create_file("tsconfig.1.json", r#"{ "include": ["dir/**/*"] }"#);

        let chain =
            TsConfigJson::resolve_extends_chain(sandbox.path().join("tsconfig.json")).unwrap();

        assert_eq!(
            chain,
            vec![
                TsConfigExtendsChain {
                    path: sandbox.path().join("tsconfig.1.json"),
                    config: TsConfigJson {
                        include: Some(vec![CompilerPath::from("dir/**/*")]),
                        ..TsConfigJson::default()
                    }
                },
                TsConfigExtendsChain {
                    path: sandbox.path().join("tsconfig.json"),
                    config: TsConfigJson {
                        extends: Some(ExtendsField::Single("./tsconfig.1.json".into())),
                        include: Some(vec![CompilerPath::from("file.tsx")]),
                        ..TsConfigJson::default()
                    }
                }
            ]
        );
    }

    #[test]
    fn resolves_multiple() {
        let sandbox = create_empty_sandbox();
        sandbox.create_file(
            "tsconfig.json",
            r#"{ "extends": ["./tsconfig.1.json", "./tsconfig.2.json"], "include": ["file.tsx"] }"#,
        );
        sandbox.create_file("tsconfig.1.json", r#"{ "include": ["dir/**/*"] }"#);
        sandbox.create_file("tsconfig.2.json", r#"{ "exclude": ["build/**/*"] }"#);

        let chain =
            TsConfigJson::resolve_extends_chain(sandbox.path().join("tsconfig.json")).unwrap();

        assert_eq!(
            chain,
            vec![
                TsConfigExtendsChain {
                    path: sandbox.path().join("tsconfig.1.json"),
                    config: TsConfigJson {
                        include: Some(vec![CompilerPath::from("dir/**/*")]),
                        ..TsConfigJson::default()
                    }
                },
                TsConfigExtendsChain {
                    path: sandbox.path().join("tsconfig.2.json"),
                    config: TsConfigJson {
                        exclude: Some(vec![CompilerPath::from("build/**/*")]),
                        ..TsConfigJson::default()
                    }
                },
                TsConfigExtendsChain {
                    path: sandbox.path().join("tsconfig.json"),
                    config: TsConfigJson {
                        extends: Some(ExtendsField::Multiple(vec![
                            "./tsconfig.1.json".into(),
                            "./tsconfig.2.json".into()
                        ])),
                        include: Some(vec![CompilerPath::from("file.tsx")]),
                        ..TsConfigJson::default()
                    }
                },
            ]
        );
    }

    #[test]
    fn resolves_chain() {
        let sandbox = create_empty_sandbox();
        sandbox.create_file(
            "tsconfig.json",
            r#"{ "extends": ["./nested/b.json", "./nested/a.json"] }"#,
        );
        sandbox.create_file("nested/a.json", r#"{ "extends": "../c.json" }"#);
        sandbox.create_file("nested/b.json", r#"{ }"#);
        sandbox.create_file("c.json", r#"{ "extends": ["package-one"] }"#);
        sandbox.create_file(
            "node_modules/package-one/tsconfig.json",
            r#"{ "extends": "@scope/package-two/tsconfig.other.json" }"#,
        );
        sandbox.create_file(
            "node_modules/@scope/package-two/tsconfig.other.json",
            r#"{ }"#,
        );

        let chain =
            TsConfigJson::resolve_extends_chain(sandbox.path().join("tsconfig.json")).unwrap();

        assert_eq!(
            chain,
            vec![
                TsConfigExtendsChain {
                    path: sandbox.path().join("nested/b.json"),
                    config: TsConfigJson::default()
                },
                TsConfigExtendsChain {
                    path: sandbox
                        .path()
                        .join("node_modules/@scope/package-two/tsconfig.other.json"),
                    config: TsConfigJson::default()
                },
                TsConfigExtendsChain {
                    path: sandbox
                        .path()
                        .join("node_modules/package-one/tsconfig.json"),
                    config: TsConfigJson {
                        extends: Some(ExtendsField::Single(
                            "@scope/package-two/tsconfig.other.json".into()
                        )),
                        ..TsConfigJson::default()
                    }
                },
                TsConfigExtendsChain {
                    path: sandbox.path().join("c.json"),
                    config: TsConfigJson {
                        extends: Some(ExtendsField::Multiple(vec!["package-one".into()])),
                        ..TsConfigJson::default()
                    }
                },
                TsConfigExtendsChain {
                    path: sandbox.path().join("nested/a.json"),
                    config: TsConfigJson {
                        extends: Some(ExtendsField::Single("../c.json".into())),
                        ..TsConfigJson::default()
                    }
                },
                TsConfigExtendsChain {
                    path: sandbox.path().join("tsconfig.json"),
                    config: TsConfigJson {
                        extends: Some(ExtendsField::Multiple(vec![
                            "./nested/b.json".into(),
                            "./nested/a.json".into()
                        ])),
                        ..TsConfigJson::default()
                    }
                },
            ]
        );
    }
}
