#[cfg(test)]
use crate::config::Config;


#[test]
fn first_default_shells() {
    let config = Config::new();
    let shells = config.shells;
    assert!(shells[0].name == String::from("pwsh"));
    assert!(shells[0].cmd == String::from("pwsh"));
    assert!(shells[0].args.len() == 3);
}

mod shells {
    use crate::config::ShellConfig;

    #[test]
    fn test_default_config() {
        let shell = ShellConfig::default();
        assert_eq!(String::from(""), shell.name);
        assert_eq!(String::from(""), shell.cmd);
        assert_eq!(0, shell.args.len());
    }
}
