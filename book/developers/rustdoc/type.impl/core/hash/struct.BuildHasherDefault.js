(function() {var type_impls = {
"jumpy":[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-BuildHasher-for-BuildHasherDefault%3CH%3E\" class=\"impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.7.0\">1.7.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/1.81.0/src/core/hash/mod.rs.html#776\">source</a></span><a href=\"#impl-BuildHasher-for-BuildHasherDefault%3CH%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;H&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/hash/trait.BuildHasher.html\" title=\"trait core::hash::BuildHasher\">BuildHasher</a> for <a class=\"struct\" href=\"https://doc.rust-lang.org/1.81.0/core/hash/struct.BuildHasherDefault.html\" title=\"struct core::hash::BuildHasherDefault\">BuildHasherDefault</a>&lt;H&gt;<div class=\"where\">where\n    H: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/hash/trait.Hasher.html\" title=\"trait core::hash::Hasher\">Hasher</a>,</div></h3></section></summary><div class=\"impl-items\"><details class=\"toggle\" open><summary><section id=\"associatedtype.Hasher\" class=\"associatedtype trait-impl\"><a href=\"#associatedtype.Hasher\" class=\"anchor\">§</a><h4 class=\"code-header\">type <a href=\"https://doc.rust-lang.org/1.81.0/core/hash/trait.BuildHasher.html#associatedtype.Hasher\" class=\"associatedtype\">Hasher</a> = H</h4></section></summary><div class='docblock'>Type of the hasher that will be created.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.build_hasher\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"https://doc.rust-lang.org/1.81.0/src/core/hash/mod.rs.html#779\">source</a><a href=\"#method.build_hasher\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.81.0/core/hash/trait.BuildHasher.html#tymethod.build_hasher\" class=\"fn\">build_hasher</a>(&amp;self) -&gt; H</h4></section></summary><div class='docblock'>Creates a new hasher. <a href=\"https://doc.rust-lang.org/1.81.0/core/hash/trait.BuildHasher.html#tymethod.build_hasher\">Read more</a></div></details></div></details>","BuildHasher","jumpy::core::physics::collisions::EntityBuildHasher","jumpy::prelude::bones_utils::prelude::fxhash::FxBuildHasher","jumpy::prelude::bones_utils::prelude::hashbrown::hash_map::DefaultHashBuilder"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-BuildHasherDefault%3CH%3E\" class=\"impl\"><a class=\"src rightside\" href=\"https://doc.rust-lang.org/1.81.0/src/core/hash/mod.rs.html#756\">source</a><a href=\"#impl-BuildHasherDefault%3CH%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;H&gt; <a class=\"struct\" href=\"https://doc.rust-lang.org/1.81.0/core/hash/struct.BuildHasherDefault.html\" title=\"struct core::hash::BuildHasherDefault\">BuildHasherDefault</a>&lt;H&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.new\" class=\"method\"><a class=\"src rightside\" href=\"https://doc.rust-lang.org/1.81.0/src/core/hash/mod.rs.html#763\">source</a><h4 class=\"code-header\">pub const fn <a href=\"https://doc.rust-lang.org/1.81.0/core/hash/struct.BuildHasherDefault.html#tymethod.new\" class=\"fn\">new</a>() -&gt; <a class=\"struct\" href=\"https://doc.rust-lang.org/1.81.0/core/hash/struct.BuildHasherDefault.html\" title=\"struct core::hash::BuildHasherDefault\">BuildHasherDefault</a>&lt;H&gt;</h4></section><span class=\"item-info\"><div class=\"stab unstable\"><span class=\"emoji\">🔬</span><span>This is a nightly-only experimental API. (<code>build_hasher_default_const_new</code>)</span></div></span></summary><div class=\"docblock\"><p>Creates a new BuildHasherDefault for Hasher <code>H</code>.</p>\n</div></details></div></details>",0,"jumpy::core::physics::collisions::EntityBuildHasher","jumpy::prelude::bones_utils::prelude::fxhash::FxBuildHasher","jumpy::prelude::bones_utils::prelude::hashbrown::hash_map::DefaultHashBuilder"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Clone-for-BuildHasherDefault%3CH%3E\" class=\"impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.7.0\">1.7.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/1.81.0/src/core/hash/mod.rs.html#785\">source</a></span><a href=\"#impl-Clone-for-BuildHasherDefault%3CH%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;H&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"https://doc.rust-lang.org/1.81.0/core/hash/struct.BuildHasherDefault.html\" title=\"struct core::hash::BuildHasherDefault\">BuildHasherDefault</a>&lt;H&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.clone\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"https://doc.rust-lang.org/1.81.0/src/core/hash/mod.rs.html#786\">source</a><a href=\"#method.clone\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.81.0/core/clone/trait.Clone.html#tymethod.clone\" class=\"fn\">clone</a>(&amp;self) -&gt; <a class=\"struct\" href=\"https://doc.rust-lang.org/1.81.0/core/hash/struct.BuildHasherDefault.html\" title=\"struct core::hash::BuildHasherDefault\">BuildHasherDefault</a>&lt;H&gt;</h4></section></summary><div class='docblock'>Returns a copy of the value. <a href=\"https://doc.rust-lang.org/1.81.0/core/clone/trait.Clone.html#tymethod.clone\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.clone_from\" class=\"method trait-impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.0.0\">1.0.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/1.81.0/src/core/clone.rs.html#172\">source</a></span><a href=\"#method.clone_from\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.81.0/core/clone/trait.Clone.html#method.clone_from\" class=\"fn\">clone_from</a>(&amp;mut self, source: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.reference.html\">&amp;Self</a>)</h4></section></summary><div class='docblock'>Performs copy-assignment from <code>source</code>. <a href=\"https://doc.rust-lang.org/1.81.0/core/clone/trait.Clone.html#method.clone_from\">Read more</a></div></details></div></details>","Clone","jumpy::core::physics::collisions::EntityBuildHasher","jumpy::prelude::bones_utils::prelude::fxhash::FxBuildHasher","jumpy::prelude::bones_utils::prelude::hashbrown::hash_map::DefaultHashBuilder"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Debug-for-BuildHasherDefault%3CH%3E\" class=\"impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.9.0\">1.9.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/1.81.0/src/core/hash/mod.rs.html#769\">source</a></span><a href=\"#impl-Debug-for-BuildHasherDefault%3CH%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;H&gt; <a class=\"trait\" href=\"jumpy/prelude/bones_utils/prelude/alloc/fmt/trait.Debug.html\" title=\"trait jumpy::prelude::bones_utils::prelude::alloc::fmt::Debug\">Debug</a> for <a class=\"struct\" href=\"https://doc.rust-lang.org/1.81.0/core/hash/struct.BuildHasherDefault.html\" title=\"struct core::hash::BuildHasherDefault\">BuildHasherDefault</a>&lt;H&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.fmt\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"https://doc.rust-lang.org/1.81.0/src/core/hash/mod.rs.html#770\">source</a><a href=\"#method.fmt\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"jumpy/prelude/bones_utils/prelude/alloc/fmt/trait.Debug.html#tymethod.fmt\" class=\"fn\">fmt</a>(&amp;self, f: &amp;mut <a class=\"struct\" href=\"jumpy/prelude/bones_utils/prelude/alloc/fmt/struct.Formatter.html\" title=\"struct jumpy::prelude::bones_utils::prelude::alloc::fmt::Formatter\">Formatter</a>&lt;'_&gt;) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.81.0/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.unit.html\">()</a>, <a class=\"struct\" href=\"jumpy/prelude/bones_utils/prelude/alloc/fmt/struct.Error.html\" title=\"struct jumpy::prelude::bones_utils::prelude::alloc::fmt::Error\">Error</a>&gt;</h4></section></summary><div class='docblock'>Formats the value using the given formatter. <a href=\"jumpy/prelude/bones_utils/prelude/alloc/fmt/trait.Debug.html#tymethod.fmt\">Read more</a></div></details></div></details>","Debug","jumpy::core::physics::collisions::EntityBuildHasher","jumpy::prelude::bones_utils::prelude::fxhash::FxBuildHasher","jumpy::prelude::bones_utils::prelude::hashbrown::hash_map::DefaultHashBuilder"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Default-for-BuildHasherDefault%3CH%3E\" class=\"impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.7.0\">1.7.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/1.81.0/src/core/hash/mod.rs.html#792\">source</a></span><a href=\"#impl-Default-for-BuildHasherDefault%3CH%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;H&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/default/trait.Default.html\" title=\"trait core::default::Default\">Default</a> for <a class=\"struct\" href=\"https://doc.rust-lang.org/1.81.0/core/hash/struct.BuildHasherDefault.html\" title=\"struct core::hash::BuildHasherDefault\">BuildHasherDefault</a>&lt;H&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.default\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"https://doc.rust-lang.org/1.81.0/src/core/hash/mod.rs.html#793\">source</a><a href=\"#method.default\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.81.0/core/default/trait.Default.html#tymethod.default\" class=\"fn\">default</a>() -&gt; <a class=\"struct\" href=\"https://doc.rust-lang.org/1.81.0/core/hash/struct.BuildHasherDefault.html\" title=\"struct core::hash::BuildHasherDefault\">BuildHasherDefault</a>&lt;H&gt;</h4></section></summary><div class='docblock'>Returns the “default value” for a type. <a href=\"https://doc.rust-lang.org/1.81.0/core/default/trait.Default.html#tymethod.default\">Read more</a></div></details></div></details>","Default","jumpy::core::physics::collisions::EntityBuildHasher","jumpy::prelude::bones_utils::prelude::fxhash::FxBuildHasher","jumpy::prelude::bones_utils::prelude::hashbrown::hash_map::DefaultHashBuilder"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-PartialEq-for-BuildHasherDefault%3CH%3E\" class=\"impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.29.0\">1.29.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/1.81.0/src/core/hash/mod.rs.html#799\">source</a></span><a href=\"#impl-PartialEq-for-BuildHasherDefault%3CH%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;H&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/cmp/trait.PartialEq.html\" title=\"trait core::cmp::PartialEq\">PartialEq</a> for <a class=\"struct\" href=\"https://doc.rust-lang.org/1.81.0/core/hash/struct.BuildHasherDefault.html\" title=\"struct core::hash::BuildHasherDefault\">BuildHasherDefault</a>&lt;H&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.eq\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"https://doc.rust-lang.org/1.81.0/src/core/hash/mod.rs.html#800\">source</a><a href=\"#method.eq\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.81.0/core/cmp/trait.PartialEq.html#tymethod.eq\" class=\"fn\">eq</a>(&amp;self, _other: &amp;<a class=\"struct\" href=\"https://doc.rust-lang.org/1.81.0/core/hash/struct.BuildHasherDefault.html\" title=\"struct core::hash::BuildHasherDefault\">BuildHasherDefault</a>&lt;H&gt;) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.bool.html\">bool</a></h4></section></summary><div class='docblock'>This method tests for <code>self</code> and <code>other</code> values to be equal, and is used\nby <code>==</code>.</div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.ne\" class=\"method trait-impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.0.0\">1.0.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/1.81.0/src/core/cmp.rs.html#262\">source</a></span><a href=\"#method.ne\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.81.0/core/cmp/trait.PartialEq.html#method.ne\" class=\"fn\">ne</a>(&amp;self, other: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.reference.html\">&amp;Rhs</a>) -&gt; <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.bool.html\">bool</a></h4></section></summary><div class='docblock'>This method tests for <code>!=</code>. The default implementation is almost always\nsufficient, and should not be overridden without very good reason.</div></details></div></details>","PartialEq","jumpy::core::physics::collisions::EntityBuildHasher","jumpy::prelude::bones_utils::prelude::fxhash::FxBuildHasher","jumpy::prelude::bones_utils::prelude::hashbrown::hash_map::DefaultHashBuilder"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-TypePath-for-BuildHasherDefault%3CAHasher%3E\" class=\"impl\"><a href=\"#impl-TypePath-for-BuildHasherDefault%3CAHasher%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl TypePath for <a class=\"struct\" href=\"https://doc.rust-lang.org/1.81.0/core/hash/struct.BuildHasherDefault.html\" title=\"struct core::hash::BuildHasherDefault\">BuildHasherDefault</a>&lt;<a class=\"struct\" href=\"jumpy/prelude/egui/ahash/struct.AHasher.html\" title=\"struct jumpy::prelude::egui::ahash::AHasher\">AHasher</a>&gt;</h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.type_path\" class=\"method trait-impl\"><a href=\"#method.type_path\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">type_path</a>() -&gt; &amp;'static <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.str.html\">str</a></h4></section></summary><div class='docblock'>Returns the fully qualified path of the underlying type. <a>Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.short_type_path\" class=\"method trait-impl\"><a href=\"#method.short_type_path\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">short_type_path</a>() -&gt; &amp;'static <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.str.html\">str</a></h4></section></summary><div class='docblock'>Returns a short, pretty-print enabled path to the type. <a>Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.type_ident\" class=\"method trait-impl\"><a href=\"#method.type_ident\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">type_ident</a>() -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.81.0/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;&amp;'static <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.str.html\">str</a>&gt;</h4></section></summary><div class='docblock'>Returns the name of the type, or <a href=\"https://doc.rust-lang.org/1.81.0/core/option/enum.Option.html#variant.None\" title=\"variant core::option::Option::None\"><code>None</code></a> if it is <a href=\"TypePath#anonymity\">anonymous</a>. <a>Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.crate_name\" class=\"method trait-impl\"><a href=\"#method.crate_name\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">crate_name</a>() -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.81.0/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;&amp;'static <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.str.html\">str</a>&gt;</h4></section></summary><div class='docblock'>Returns the name of the crate the type is in, or <a href=\"https://doc.rust-lang.org/1.81.0/core/option/enum.Option.html#variant.None\" title=\"variant core::option::Option::None\"><code>None</code></a> if it is <a href=\"TypePath#anonymity\">anonymous</a>. <a>Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.module_path\" class=\"method trait-impl\"><a href=\"#method.module_path\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a class=\"fn\">module_path</a>() -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.81.0/core/option/enum.Option.html\" title=\"enum core::option::Option\">Option</a>&lt;&amp;'static <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.str.html\">str</a>&gt;</h4></section></summary><div class='docblock'>Returns the path to the moudle the type is in, or <a href=\"https://doc.rust-lang.org/1.81.0/core/option/enum.Option.html#variant.None\" title=\"variant core::option::Option::None\"><code>None</code></a> if it is <a href=\"TypePath#anonymity\">anonymous</a>. <a>Read more</a></div></details></div></details>","TypePath","jumpy::prelude::bones_utils::prelude::hashbrown::hash_map::DefaultHashBuilder"],["<section id=\"impl-Eq-for-BuildHasherDefault%3CH%3E\" class=\"impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.29.0\">1.29.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/1.81.0/src/core/hash/mod.rs.html#806\">source</a></span><a href=\"#impl-Eq-for-BuildHasherDefault%3CH%3E\" class=\"anchor\">§</a><h3 class=\"code-header\">impl&lt;H&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/cmp/trait.Eq.html\" title=\"trait core::cmp::Eq\">Eq</a> for <a class=\"struct\" href=\"https://doc.rust-lang.org/1.81.0/core/hash/struct.BuildHasherDefault.html\" title=\"struct core::hash::BuildHasherDefault\">BuildHasherDefault</a>&lt;H&gt;</h3></section>","Eq","jumpy::core::physics::collisions::EntityBuildHasher","jumpy::prelude::bones_utils::prelude::fxhash::FxBuildHasher","jumpy::prelude::bones_utils::prelude::hashbrown::hash_map::DefaultHashBuilder"]]
};if (window.register_type_impls) {window.register_type_impls(type_impls);} else {window.pending_type_impls = type_impls;}})()