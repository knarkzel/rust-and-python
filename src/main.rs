use rustpython_vm as vm;

fn main() -> vm::PyResult<()> {
    let python = r#"
print("Hello World!")

def triple(x):
    return x * 3

print(triple(10))
    "#;
    
    vm::Interpreter::without_stdlib(Default::default()).enter(|vm| {
        let scope = vm.new_scope_with_builtins();

        let code_obj = vm
            .compile(
                python,
                vm::compiler::Mode::Exec,
                "<embedded>".to_owned(),
            )
            .map_err(|err| vm.new_syntax_error(&err))?;

        vm.run_code_obj(code_obj, scope)?;

        Ok(())
    })
}
