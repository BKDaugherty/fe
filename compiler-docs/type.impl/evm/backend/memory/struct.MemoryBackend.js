(function() {var type_impls = {
"fe_compiler_test_utils":[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-MemoryBackend%3C'vicinity%3E\" class=\"impl\"><a href=\"#impl-MemoryBackend%3C'vicinity%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;'vicinity&gt; MemoryBackend&lt;'vicinity&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.new\" class=\"method\"><h4 class=\"code-header\">pub fn <a class=\"fn\">new</a>(\n    vicinity: &amp;'vicinity MemoryVicinity,\n    state: <a class=\"struct\" href=\"https://doc.rust-lang.org/1.75.0/alloc/collections/btree/map/struct.BTreeMap.html\" title=\"struct alloc::collections::btree::map::BTreeMap\">BTreeMap</a>&lt;H160, MemoryAccount&gt;\n) -&gt; MemoryBackend&lt;'vicinity&gt;</h4></section></summary><div class=\"docblock\"><p>Create a new memory backend.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.state\" class=\"method\"><h4 class=\"code-header\">pub fn <a class=\"fn\">state</a>(&amp;self) -&gt; &amp;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.75.0/alloc/collections/btree/map/struct.BTreeMap.html\" title=\"struct alloc::collections::btree::map::BTreeMap\">BTreeMap</a>&lt;H160, MemoryAccount&gt;</h4></section></summary><div class=\"docblock\"><p>Get the underlying <code>BTreeMap</code> storing the state.</p>\n</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.state_mut\" class=\"method\"><h4 class=\"code-header\">pub fn <a class=\"fn\">state_mut</a>(&amp;mut self) -&gt; &amp;mut <a class=\"struct\" href=\"https://doc.rust-lang.org/1.75.0/alloc/collections/btree/map/struct.BTreeMap.html\" title=\"struct alloc::collections::btree::map::BTreeMap\">BTreeMap</a>&lt;H160, MemoryAccount&gt;</h4></section></summary><div class=\"docblock\"><p>Get a mutable reference to the underlying <code>BTreeMap</code> storing the state.</p>\n</div></details></div></details>",0,"fe_compiler_test_utils::Backend"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Debug-for-MemoryBackend%3C'vicinity%3E\" class=\"impl\"><a href=\"#impl-Debug-for-MemoryBackend%3C'vicinity%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;'vicinity&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/fmt/trait.Debug.html\" title=\"trait core::fmt::Debug\">Debug</a> for MemoryBackend&lt;'vicinity&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.fmt\" class=\"method trait-impl\"><a href=\"#method.fmt\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.75.0/core/fmt/trait.Debug.html#tymethod.fmt\" class=\"fn\">fmt</a>(&amp;self, f: &amp;mut <a class=\"struct\" href=\"https://doc.rust-lang.org/1.75.0/core/fmt/struct.Formatter.html\" title=\"struct core::fmt::Formatter\">Formatter</a>&lt;'_&gt;) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.75.0/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.75.0/std/primitive.unit.html\">()</a>, <a class=\"struct\" href=\"https://doc.rust-lang.org/1.75.0/core/fmt/struct.Error.html\" title=\"struct core::fmt::Error\">Error</a>&gt;</h4></section></summary><div class='docblock'>Formats the value using the given formatter. <a href=\"https://doc.rust-lang.org/1.75.0/core/fmt/trait.Debug.html#tymethod.fmt\">Read more</a></div></details></div></details>","Debug","fe_compiler_test_utils::Backend"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-ApplyBackend-for-MemoryBackend%3C'vicinity%3E\" class=\"impl\"><a href=\"#impl-ApplyBackend-for-MemoryBackend%3C'vicinity%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;'vicinity&gt; ApplyBackend for MemoryBackend&lt;'vicinity&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.apply\" class=\"method trait-impl\"><a href=\"#method.apply\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">apply</a>&lt;A, I, L&gt;(&amp;mut self, values: A, logs: L, delete_empty: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.75.0/std/primitive.bool.html\">bool</a>)<span class=\"where fmt-newline\">where\n    A: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/iter/traits/collect/trait.IntoIterator.html\" title=\"trait core::iter::traits::collect::IntoIterator\">IntoIterator</a>&lt;Item = Apply&lt;I&gt;&gt;,\n    I: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/iter/traits/collect/trait.IntoIterator.html\" title=\"trait core::iter::traits::collect::IntoIterator\">IntoIterator</a>&lt;Item = (H256, H256)&gt;,\n    L: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/iter/traits/collect/trait.IntoIterator.html\" title=\"trait core::iter::traits::collect::IntoIterator\">IntoIterator</a>&lt;Item = Log&gt;,</span></h4></section></summary><div class='docblock'>Apply given values and logs at backend.</div></details></div></details>","ApplyBackend","fe_compiler_test_utils::Backend"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Clone-for-MemoryBackend%3C'vicinity%3E\" class=\"impl\"><a href=\"#impl-Clone-for-MemoryBackend%3C'vicinity%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;'vicinity&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.75.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for MemoryBackend&lt;'vicinity&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.clone\" class=\"method trait-impl\"><a href=\"#method.clone\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.75.0/core/clone/trait.Clone.html#tymethod.clone\" class=\"fn\">clone</a>(&amp;self) -&gt; MemoryBackend&lt;'vicinity&gt;</h4></section></summary><div class='docblock'>Returns a copy of the value. <a href=\"https://doc.rust-lang.org/1.75.0/core/clone/trait.Clone.html#tymethod.clone\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.clone_from\" class=\"method trait-impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.0.0\">1.0.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/1.75.0/src/core/clone.rs.html#169\">source</a></span><a href=\"#method.clone_from\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.75.0/core/clone/trait.Clone.html#method.clone_from\" class=\"fn\">clone_from</a>(&amp;mut self, source: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.75.0/std/primitive.reference.html\">&amp;Self</a>)</h4></section></summary><div class='docblock'>Performs copy-assignment from <code>source</code>. <a href=\"https://doc.rust-lang.org/1.75.0/core/clone/trait.Clone.html#method.clone_from\">Read more</a></div></details></div></details>","Clone","fe_compiler_test_utils::Backend"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Backend-for-MemoryBackend%3C'vicinity%3E\" class=\"impl\"><a href=\"#impl-Backend-for-MemoryBackend%3C'vicinity%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;'vicinity&gt; Backend for MemoryBackend&lt;'vicinity&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.gas_price\" class=\"method trait-impl\"><a href=\"#method.gas_price\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">gas_price</a>(&amp;self) -&gt; U256</h4></section></summary><div class='docblock'>Gas price. Unused for London.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.origin\" class=\"method trait-impl\"><a href=\"#method.origin\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">origin</a>(&amp;self) -&gt; H160</h4></section></summary><div class='docblock'>Origin.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.block_hash\" class=\"method trait-impl\"><a href=\"#method.block_hash\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">block_hash</a>(&amp;self, number: U256) -&gt; H256</h4></section></summary><div class='docblock'>Environmental block hash.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.block_number\" class=\"method trait-impl\"><a href=\"#method.block_number\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">block_number</a>(&amp;self) -&gt; U256</h4></section></summary><div class='docblock'>Environmental block number.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.block_coinbase\" class=\"method trait-impl\"><a href=\"#method.block_coinbase\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">block_coinbase</a>(&amp;self) -&gt; H160</h4></section></summary><div class='docblock'>Environmental coinbase.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.block_timestamp\" class=\"method trait-impl\"><a href=\"#method.block_timestamp\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">block_timestamp</a>(&amp;self) -&gt; U256</h4></section></summary><div class='docblock'>Environmental block timestamp.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.block_difficulty\" class=\"method trait-impl\"><a href=\"#method.block_difficulty\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">block_difficulty</a>(&amp;self) -&gt; U256</h4></section></summary><div class='docblock'>Environmental block difficulty.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.block_gas_limit\" class=\"method trait-impl\"><a href=\"#method.block_gas_limit\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">block_gas_limit</a>(&amp;self) -&gt; U256</h4></section></summary><div class='docblock'>Environmental block gas limit.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.block_base_fee_per_gas\" class=\"method trait-impl\"><a href=\"#method.block_base_fee_per_gas\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">block_base_fee_per_gas</a>(&amp;self) -&gt; U256</h4></section></summary><div class='docblock'>Environmental block base fee.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.chain_id\" class=\"method trait-impl\"><a href=\"#method.chain_id\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">chain_id</a>(&amp;self) -&gt; U256</h4></section></summary><div class='docblock'>Environmental chain ID.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.exists\" class=\"method trait-impl\"><a href=\"#method.exists\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">exists</a>(&amp;self, address: H160) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.75.0/std/primitive.bool.html\">bool</a></h4></section></summary><div class='docblock'>Whether account at address exists.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.basic\" class=\"method trait-impl\"><a href=\"#method.basic\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">basic</a>(&amp;self, address: H160) -&gt; Basic</h4></section></summary><div class='docblock'>Get basic account information.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.code\" class=\"method trait-impl\"><a href=\"#method.code\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">code</a>(&amp;self, address: H160) -&gt; <a class=\"struct\" href=\"https://doc.rust-lang.org/1.75.0/alloc/vec/struct.Vec.html\" title=\"struct alloc::vec::Vec\">Vec</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.75.0/std/primitive.u8.html\">u8</a>&gt; <a href=\"#\" class=\"tooltip\" data-notable-ty=\"Vec&lt;u8&gt;\">ⓘ</a></h4></section></summary><div class='docblock'>Get account code.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.storage\" class=\"method trait-impl\"><a href=\"#method.storage\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">storage</a>(&amp;self, address: H160, index: H256) -&gt; H256</h4></section></summary><div class='docblock'>Get storage value of address at index.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.original_storage\" class=\"method trait-impl\"><a href=\"#method.original_storage\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">original_storage</a>(&amp;self, address: H160, index: H256) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.75.0/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;H256&gt;</h4></section></summary><div class='docblock'>Get original storage value of address at index, if available.</div></details></div></details>","Backend","fe_compiler_test_utils::Backend"]]
};if (window.register_type_impls) {window.register_type_impls(type_impls);} else {window.pending_type_impls = type_impls;}})()