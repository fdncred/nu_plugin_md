use nu_plugin::{serve_plugin, EvaluatedCall, LabeledError, MsgPackSerializer, Plugin};
use nu_protocol::{Category, PluginExample, PluginSignature, Span, Spanned, SyntaxShape, Value};
use termimad::*;

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

        test0(arg.clone(), find.span);
        test1(arg.clone(), find.span);
        test2(arg.clone(), find.span);
        test3(arg.clone(), find.span);
        test4(arg.clone(), find.span);

        Ok(Value::test_nothing())
    }
}

fn main() {
    serve_plugin(&mut Implementation::new(), MsgPackSerializer);
}

fn test0(arg: String, _span: Span) {
    let line = termimad::inline(&arg);
    eprintln!("test0:\n{}", line);
}

fn test1(arg: String, _span: Span) {
    let skin = MadSkin::default();
    eprintln!("test1:\n{}", skin.term_text(&arg));
}
fn test2(arg: String, _span: Span) {
    // meant to format this nicely
    // let s = <below>
    // md $s

    // ----

    // # Centered Title

    // A medium long text.
    // It's bigger than the other parts but thinner than your terminal.
    // *I mean I hope it's thinner than your terminal*

    //     A right aligned thing

    // ----

    // Note how all parts are aligned with the content width:
    // * title's centering is consistent with the text
    // * horizontal separators aren't wider than the text
    // * right aligned thing isn't stuck to the terminal's right side

    // This content align trick is useful for wide terminals
    // (especially when you know the content is thin)

    // ----
    let mut skin = MadSkin::default();
    skin.code_block.align = Alignment::Right;
    let (width, _) = terminal_size();
    let terminal_width = width as usize;
    let mut text = FmtText::from(&skin, &arg, Some(terminal_width));
    text.set_rendering_width(text.content_width());
    eprintln!("test2:\n{}", text);
}

fn test3(arg: String, _span: Span) {
    let skin = MadSkin::default();
    // let my_markdown = "#title\n* item 1\n* item 2";
    let text = FmtText::from(&skin, &arg, Some(80));
    eprintln!("test3:\n{}", text);
}

fn test4(arg: String, _span: Span) {
    use termimad::minimad::{OwningTemplateExpander, TextTemplate};
    static TEMPLATE: &str = r#"
----
# ${title}

## When to use it ?

* ${points}

## Terminal capabilities:

|:-:|:-:|
|**Capability**|**Necessary**|
|-:|:-:|
|ansi escape codes|${ansi-codes}|
|non ascii characters|${non-ascii}|
|-:|:-:|

## Skin initialization:

```
${code}
```

"#;

    fn fun(
        title: &str,
        skin: MadSkin,
        when: &[&str],
        ansi_codes: bool,
        non_ascii: bool,
        code: &str,
    ) {
        let mut expander = OwningTemplateExpander::new();
        expander
            .set("title", title)
            .set_lines("points", when.join("\n"))
            .set_lines("code", code)
            .set("ansi-codes", if ansi_codes { "yes" } else { "no" })
            .set("non-ascii", if non_ascii { "yes" } else { "no" });
        let template = TextTemplate::from(TEMPLATE);
        let text = expander.expand(&template);
        let (width, _) = terminal_size();
        let fmt_text = FmtText::from_text(&skin, text, Some(width as usize));
        eprintln!("{}", fmt_text);
    }
    // default skin
    let skin = MadSkin::default();
    fun(
        "Default skin",
        skin,
        &["almost always"],
        true,
        true,
        "let skin = MadSkin::default();",
    );

    // skin without ANSI escape codes
    let skin = MadSkin::no_style();
    fun(
        "Without ANSI escape codes",
        skin,
        &["when your terminal is very old"],
        false,
        true,
        "let skin = MadSkin::no_style();",
    );

    // skin with only ascii chars
    let mut skin = MadSkin::default();
    skin.limit_to_ascii();
    fun(
        "Using only ASCII",
        skin,
        &["when your terminal only knows ASCII"],
        true,
        false,
        r#"
        let mut skin = MadSkin::default();
        skin.limit_to_ascii();
        "#,
    );

    // skin with only ascii chars and no ANSI escape code
    let mut skin = MadSkin::no_style();
    skin.limit_to_ascii();
    fun(
        "Using only ASCII and no ANSI escape code",
        skin,
        &[
            "when your terminal is very very very old",
            "when your multiplexer is pigeon carrier based",
        ],
        false,
        false,
        r#"
        let mut skin = MadSkin::no_style();
        skin.limit_to_ascii();
        "#,
    );

    // eprintln!("test4:\n{}", text);
}
