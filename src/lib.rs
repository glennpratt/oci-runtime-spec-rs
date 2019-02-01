#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate derive_builder;

pub mod spec;

#[cfg(test)]
mod tests {
    use crate::spec::*;
    use glob::glob;
    use serde_json;
    use std::fs::File;

    #[test]
    fn builder() -> Result<(), String> {
        ConfigSchemaBuilder::default()
            .oci_version("1.0.0")
            .process(
                ProcessBuilder::default()
                    .args(vec!["sleep".to_string(), "1".to_string()])
                    .cwd("/root")
                    .build()?,
            )
            .build()?;
        Ok(())
    }

    #[test]
    fn config_good() {
        let mut count = 0;
        for entry in glob("../runtime-spec/schema/test/config/good/*.json").unwrap() {
            let entry = entry.unwrap();
            let msg = format!("good file {}", &entry.display());
            let json = File::open(&entry).expect(&format!("opening {}", &msg));
            let _config: ConfigSchema =
                serde_json::from_reader(&json).expect(&format!("parsing {:?}", &msg));
            count += 1;
        }
        assert_eq!(4, count);
    }

    #[test]
    fn config_bad() {
        let mut count = 0;
        for entry in glob("../runtime-spec/schema/test/config/bad/*.json").unwrap() {
            let entry = entry.unwrap();
            let msg = format!("bad file {}", &entry.display());
            let json = File::open(&entry).expect(&format!("opening {}", &msg));
            let res: Result<ConfigSchema, serde_json::Error> = serde_json::from_reader(&json);
            res.expect_err(&format!("should not parse {:?}", &msg));
            count += 1;
        }
        assert_eq!(2, count);
    }
    #[test]
    fn state_good() {
        let mut count = 0;
        for entry in glob("../runtime-spec/schema/test/state/good/*.json").unwrap() {
            let entry = entry.unwrap();
            let msg = format!("good file {}", &entry.display());
            let json = File::open(&entry).expect(&format!("opening {}", &msg));
            let _state: StateSchema =
                serde_json::from_reader(&json).expect(&format!("parsing {:?}", &msg));
            count += 1;
        }
        assert_eq!(1, count);
    }

    #[test]
    fn state_bad() {
        let mut count = 0;
        for entry in glob("../runtime-spec/schema/test/state/bad/*.json").unwrap() {
            let entry = entry.unwrap();
            let msg = format!("bad file {}", &entry.display());
            let json = File::open(&entry).expect(&format!("opening {}", &msg));
            let res: Result<StateSchema, serde_json::Error> = serde_json::from_reader(&json);
            res.expect_err(&format!("should not parse {:?}", &msg));
            count += 1;
        }
        assert_eq!(1, count);
    }
}
