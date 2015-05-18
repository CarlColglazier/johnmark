//! Tests based on commonmark examples.

extern crate johnmark;

use johnmark::convert;

#[test]
#[ignore]
fn tab_expansion() {
    assert_eq!("<pre><code>foo baz     bim</code></pre>", convert("\tfoo\tbaz\t\tbim"));
}

#[test]
// TODO: Fix compatibility.
fn blank_lines() {
    assert_eq!("<p>aaa</p><h1>aaa</h1>", convert("  \n\naaa\n  \n\n# aaa\n\n  "));
}

#[test]
#[ignore]
// TODO: Write tests.
fn container_blocks() {}

#[test]
#[ignore]
// TODO: Write tests.
fn lists() {}

#[test]
#[ignore]
// TODO: Write tests.
fn inlines() {}

#[test]
#[ignore]
// TODO: Write tests.
fn entities() {}

#[test]
#[ignore]
// TODO: Write tests.
fn emphasis_and_strong_emphasis() {}

#[test]
#[ignore]
// TODO: Write tests.
fn links() {}

#[test]
#[ignore]
// TODO: Write tests.
fn images() {}

#[test]
#[ignore]
// TODO: Write tests.
fn autolinks() {}

#[test]
#[ignore]
// TODO: Write tests.
fn raw_html() {}

#[test]
#[ignore]
// TODO: Finish adding tests and make compatible.
fn hard_line_breaks() {
    // http://spec.commonmark.org/0.19/#example-529
    assert_eq!("<p>foo<br />\nbaz</p>", convert("foo  \nbaz"));

    // http://spec.commonmark.org/0.19/#example-530
    assert_eq!("<p>foo<br />\nbaz</p>", convert("foo       \nbaz"));
}

#[test]
#[ignore]
// TODO
fn soft_line_breaks() {
    // http://spec.commonmark.org/0.19/#example-544
    assert_eq!("<p>foo\nbaz</p>", convert("foo\nbaz"));

    // http://spec.commonmark.org/0.19/#example-545
    assert_eq!("<p>foo\nbaz</p>", convert("foo \n baz"));
}

#[test]
fn textual_content() {
    // http://spec.commonmark.org/0.19/#example-546
    assert_eq!("<p>hello $.;'there</p>", convert("hello $.;'there"));

    // http://spec.commonmark.org/0.19/#example-547
    // TODO
    //assert_eq!("<p>Foo χρῆν</p>", convert("Foo χρῆν"));

    // http://spec.commonmark.org/0.19/#example-548
    assert_eq!("<p>Multiple     spaces</p>", convert("Multiple     spaces"));

}
