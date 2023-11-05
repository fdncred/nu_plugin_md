use nu_plugin::{serve_plugin, EvaluatedCall, LabeledError, MsgPackSerializer, Plugin};
use nu_protocol::{Category, PluginExample, PluginSignature, Spanned, SyntaxShape, Value};

struct Implementation;

impl Implementation {
    fn new() -> Self {
        Self {}
    }
}

impl Plugin for Implementation {
    fn signature(&self) -> Vec<PluginSignature> {
        vec![PluginSignature::build("md")
            .usage("View md results")
            .required("markdown", SyntaxShape::String, "markdown to render")
            .category(Category::Experimental)
            .plugin_examples(vec![PluginExample {
                description: "This is the example descripion".into(),
                example: "some pipeline involving md".into(),
                result: None,
            }])]
    }

    fn run(
        &mut self,
        name: &str,
        call: &EvaluatedCall,
        _input: &Value,
    ) -> Result<Value, LabeledError> {
        assert_eq!(name, "md");
        let find: Spanned<String> = call.req(0)?;
        let arg = find.item;

        let skin = termimad::MadSkin::default();
        let line = termimad::inline(&arg);
        let ret_avl = Value::string(line.to_string(), find.span);
        eprintln!("{}", skin.term_text(&arg));

        Ok(ret_avl)
    }
}

fn main() {
    serve_plugin(&mut Implementation::new(), MsgPackSerializer);
}
