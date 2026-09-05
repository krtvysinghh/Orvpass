pub fn generate_shell_init() {
    println!(r#"# Orvpass Shell Fast Shortcuts
alias op='orvpass'
alias opg='orvpass get'
alias opc='orvpass cp'
alias opcu='orvpass cpu'
alias opct='orvpass cpt'
alias opl='orvpass list'
alias opgen='orvpass generate'
"#);
}
