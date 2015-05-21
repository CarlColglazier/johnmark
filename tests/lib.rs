extern crate johnmark;

use johnmark::convert;

#[test]
fn extra_characters() {
    assert_eq!("<p>foo</p>", convert("   foo"));
    assert_eq!("<p>foo\nbar</p>", convert("foo\n   bar"));
}

#[test]
fn header_paragraph() {
    assert_eq!("<h2>Header</h2>", convert("## Header"));
    assert_eq!("<h1>Header</h1>", convert("Header\n==="));
    assert_eq!("<h2>Header</h2>", convert("Header\n---"));
    assert_eq!("<h1>Header</h1><h1>Header</h1>", convert("Header\n===\n\n# Header"));
    assert_eq!("<p>Content</p>", convert("Content"));
    assert_eq!("<h5>Header</h5><p>Content</p>", convert("##### Header\n\nContent"));
    assert_eq!("<p>Content\n</p><h1>Header</h1>", convert("Content\n# Header"));
}
/*
#[test]
fn blockquote_paragraph() {
    assert_eq!("<blockquote><p>Quote</p></blockquote>", convert(">Quote"));
    assert_eq!("<p>foo</p><blockquote><p>bar</p></blockquote>", convert("foo\n>bar"))
}
*/
#[test]
fn emphasis() {
    assert_eq!("<p><strong>bold</strong></p>", convert("**bold**"));
    assert_eq!("<p><em>bold</em></p>", convert("*bold*"));
    assert_eq!("<p><strong><em>bold</em></strong></p>", convert("***bold***"));
    assert_eq!("<p>*<strong><em>bold</em></strong>*</p>", convert("****bold****"));

    // Probably will not happen, but you never know!
    assert_eq!("<p>****<strong><em>bold</em></strong>****</p>", convert("*******bold*******"));

    // Unclosed
    assert_eq!("<p>**bold*</p>", convert("**bold*"));
}

#[test]
fn code_block() {
    assert_eq!("<pre><code>Test</code></pre>", convert("\tTest"));
    assert_eq!("<pre><code>Foo\n</code></pre><p>Paragraph</p>", convert("\tFoo\nParagraph"));
    assert_eq!("<pre><code>Foo\n</code></pre><p>Paragraph</p>", convert("\tFoo\nParagraph"));
    assert_eq!("<pre><code>Foo\nBar</code></pre>", convert("\tFoo\n\tBar"));

    // Using spaces.
    assert_eq!("<pre><code>Foo</code></pre>", convert("    Foo"));
    assert_eq!("<pre><code>Foo\nBar</code></pre>", convert("    Foo\n    Bar"));
    assert_eq!("<pre><code>Foo\n</code></pre><p>Bar</p>", convert("    Foo\nBar"));
}

#[test]
fn inline_code() {
    assert_eq!("<p>Some <code>*code*</code></p>", convert("Some `*code*`"));
}
