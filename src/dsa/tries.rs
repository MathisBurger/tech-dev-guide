use trie_rs::TrieBuilder;


/// Tries are trees but usually used for strings.
/// A common example is a software that checks if a string max contain a typo
/// Every char represents a node in the tree.
/// If you split now the word you inserted into the chars and checks if each of them
/// follows the chaining of the tree you can see if the word is valid.
/// Just watch this video for better explanation:
/// https://www.youtube.com/watch?v=kMt9Y5fv4Ug&ab_channel=GoogleStudents
///
/// NOTE: This implementation is not implemented by myself, because the logic behind this
/// is only for visualization.
pub fn tries() {
    let mut builder = TrieBuilder::new();  // Inferred `TrieBuilder<u8>` automatically
    builder.push("すし");
    builder.push("すしや");
    builder.push("すしだね");
    builder.push("すしづめ");
    builder.push("すしめし");
    builder.push("すしをにぎる");
    builder.push("すし");  // Word `push`ed twice is just ignored.
    builder.push("🍣");

    let trie = builder.build();

    assert_eq!(trie.exact_match("すし"), true);
    assert_eq!(trie.exact_match("🍣"), true);
    assert_eq!(trie.exact_match("🍜"), false);
}