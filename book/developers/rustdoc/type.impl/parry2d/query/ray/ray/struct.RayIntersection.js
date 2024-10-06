(function() {var type_impls = {
"jumpy":[["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Clone-for-RayIntersection\" class=\"impl\"><a class=\"src rightside\" href=\"http://docs.rs/parry/0.1.1/src/parry2d/query/ray/ray.rs.html#63\">source</a><a href=\"#impl-Clone-for-RayIntersection\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a> for <a class=\"struct\" href=\"http://docs.rs/parry/0.1.1/parry2d/query/ray/ray/struct.RayIntersection.html\" title=\"struct parry2d::query::ray::ray::RayIntersection\">RayIntersection</a></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.clone\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"http://docs.rs/parry/0.1.1/src/parry2d/query/ray/ray.rs.html#63\">source</a><a href=\"#method.clone\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.81.0/core/clone/trait.Clone.html#tymethod.clone\" class=\"fn\">clone</a>(&amp;self) -&gt; <a class=\"struct\" href=\"http://docs.rs/parry/0.1.1/parry2d/query/ray/ray/struct.RayIntersection.html\" title=\"struct parry2d::query::ray::ray::RayIntersection\">RayIntersection</a></h4></section></summary><div class='docblock'>Returns a copy of the value. <a href=\"https://doc.rust-lang.org/1.81.0/core/clone/trait.Clone.html#tymethod.clone\">Read more</a></div></details><details class=\"toggle method-toggle\" open><summary><section id=\"method.clone_from\" class=\"method trait-impl\"><span class=\"rightside\"><span class=\"since\" title=\"Stable since Rust version 1.0.0\">1.0.0</span> · <a class=\"src\" href=\"https://doc.rust-lang.org/1.81.0/src/core/clone.rs.html#172\">source</a></span><a href=\"#method.clone_from\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"https://doc.rust-lang.org/1.81.0/core/clone/trait.Clone.html#method.clone_from\" class=\"fn\">clone_from</a>(&amp;mut self, source: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.reference.html\">&amp;Self</a>)</h4></section></summary><div class='docblock'>Performs copy-assignment from <code>source</code>. <a href=\"https://doc.rust-lang.org/1.81.0/core/clone/trait.Clone.html#method.clone_from\">Read more</a></div></details></div></details>","Clone","jumpy::core::physics::rapier::RayIntersection"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-Debug-for-RayIntersection\" class=\"impl\"><a class=\"src rightside\" href=\"http://docs.rs/parry/0.1.1/src/parry2d/query/ray/ray.rs.html#63\">source</a><a href=\"#impl-Debug-for-RayIntersection\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"jumpy/prelude/bones_utils/prelude/alloc/fmt/trait.Debug.html\" title=\"trait jumpy::prelude::bones_utils::prelude::alloc::fmt::Debug\">Debug</a> for <a class=\"struct\" href=\"http://docs.rs/parry/0.1.1/parry2d/query/ray/ray/struct.RayIntersection.html\" title=\"struct parry2d::query::ray::ray::RayIntersection\">RayIntersection</a></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.fmt\" class=\"method trait-impl\"><a class=\"src rightside\" href=\"http://docs.rs/parry/0.1.1/src/parry2d/query/ray/ray.rs.html#63\">source</a><a href=\"#method.fmt\" class=\"anchor\">§</a><h4 class=\"code-header\">fn <a href=\"jumpy/prelude/bones_utils/prelude/alloc/fmt/trait.Debug.html#tymethod.fmt\" class=\"fn\">fmt</a>(&amp;self, f: &amp;mut <a class=\"struct\" href=\"jumpy/prelude/bones_utils/prelude/alloc/fmt/struct.Formatter.html\" title=\"struct jumpy::prelude::bones_utils::prelude::alloc::fmt::Formatter\">Formatter</a>&lt;'_&gt;) -&gt; <a class=\"enum\" href=\"https://doc.rust-lang.org/1.81.0/core/result/enum.Result.html\" title=\"enum core::result::Result\">Result</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.unit.html\">()</a>, <a class=\"struct\" href=\"jumpy/prelude/bones_utils/prelude/alloc/fmt/struct.Error.html\" title=\"struct jumpy::prelude::bones_utils::prelude::alloc::fmt::Error\">Error</a>&gt;</h4></section></summary><div class='docblock'>Formats the value using the given formatter. <a href=\"jumpy/prelude/bones_utils/prelude/alloc/fmt/trait.Debug.html#tymethod.fmt\">Read more</a></div></details></div></details>","Debug","jumpy::core::physics::rapier::RayIntersection"],["<details class=\"toggle implementors-toggle\" open><summary><section id=\"impl-RayIntersection\" class=\"impl\"><a class=\"src rightside\" href=\"http://docs.rs/parry/0.1.1/src/parry2d/query/ray/ray.rs.html#90\">source</a><a href=\"#impl-RayIntersection\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"struct\" href=\"http://docs.rs/parry/0.1.1/parry2d/query/ray/ray/struct.RayIntersection.html\" title=\"struct parry2d::query::ray::ray::RayIntersection\">RayIntersection</a></h3></section></summary><div class=\"impl-items\"><details class=\"toggle method-toggle\" open><summary><section id=\"method.new\" class=\"method\"><a class=\"src rightside\" href=\"http://docs.rs/parry/0.1.1/src/parry2d/query/ray/ray.rs.html#105\">source</a><h4 class=\"code-header\">pub fn <a href=\"http://docs.rs/parry/0.1.1/parry2d/query/ray/ray/struct.RayIntersection.html#tymethod.new\" class=\"fn\">new</a>(\n    toi: <a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.f32.html\">f32</a>,\n    normal: <a class=\"struct\" href=\"jumpy/core/physics/rapier/nalgebra/struct.Matrix.html\" title=\"struct jumpy::core::physics::rapier::nalgebra::Matrix\">Matrix</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.f32.html\">f32</a>, <a class=\"struct\" href=\"jumpy/core/physics/rapier/nalgebra/struct.Const.html\" title=\"struct jumpy::core::physics::rapier::nalgebra::Const\">Const</a>&lt;2&gt;, <a class=\"struct\" href=\"jumpy/core/physics/rapier/nalgebra/struct.Const.html\" title=\"struct jumpy::core::physics::rapier::nalgebra::Const\">Const</a>&lt;1&gt;, <a class=\"struct\" href=\"jumpy/core/physics/rapier/nalgebra/struct.ArrayStorage.html\" title=\"struct jumpy::core::physics::rapier::nalgebra::ArrayStorage\">ArrayStorage</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.f32.html\">f32</a>, 2, 1&gt;&gt;,\n    feature: <a class=\"enum\" href=\"jumpy/core/physics/rapier/enum.FeatureId.html\" title=\"enum jumpy::core::physics::rapier::FeatureId\">FeatureId</a>,\n) -&gt; <a class=\"struct\" href=\"http://docs.rs/parry/0.1.1/parry2d/query/ray/ray/struct.RayIntersection.html\" title=\"struct parry2d::query::ray::ray::RayIntersection\">RayIntersection</a></h4></section></summary><div class=\"docblock\"><p>Creates a new <code>RayIntersection</code>.</p>\n</div></details><section id=\"method.transform_by\" class=\"method\"><a class=\"src rightside\" href=\"http://docs.rs/parry/0.1.1/src/parry2d/query/ray/ray.rs.html#114\">source</a><h4 class=\"code-header\">pub fn <a href=\"http://docs.rs/parry/0.1.1/parry2d/query/ray/ray/struct.RayIntersection.html#tymethod.transform_by\" class=\"fn\">transform_by</a>(\n    &amp;self,\n    transform: &amp;<a class=\"struct\" href=\"jumpy/core/physics/rapier/nalgebra/struct.Isometry.html\" title=\"struct jumpy::core::physics::rapier::nalgebra::Isometry\">Isometry</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.f32.html\">f32</a>, <a class=\"struct\" href=\"jumpy/core/physics/rapier/nalgebra/struct.Unit.html\" title=\"struct jumpy::core::physics::rapier::nalgebra::Unit\">Unit</a>&lt;<a class=\"struct\" href=\"jumpy/core/physics/rapier/nalgebra/struct.Complex.html\" title=\"struct jumpy::core::physics::rapier::nalgebra::Complex\">Complex</a>&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.81.0/std/primitive.f32.html\">f32</a>&gt;&gt;, 2&gt;,\n) -&gt; <a class=\"struct\" href=\"http://docs.rs/parry/0.1.1/parry2d/query/ray/ray/struct.RayIntersection.html\" title=\"struct parry2d::query::ray::ray::RayIntersection\">RayIntersection</a></h4></section></div></details>",0,"jumpy::core::physics::rapier::RayIntersection"],["<section id=\"impl-Copy-for-RayIntersection\" class=\"impl\"><a class=\"src rightside\" href=\"http://docs.rs/parry/0.1.1/src/parry2d/query/ray/ray.rs.html#63\">source</a><a href=\"#impl-Copy-for-RayIntersection\" class=\"anchor\">§</a><h3 class=\"code-header\">impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.81.0/core/marker/trait.Copy.html\" title=\"trait core::marker::Copy\">Copy</a> for <a class=\"struct\" href=\"http://docs.rs/parry/0.1.1/parry2d/query/ray/ray/struct.RayIntersection.html\" title=\"struct parry2d::query::ray::ray::RayIntersection\">RayIntersection</a></h3></section>","Copy","jumpy::core::physics::rapier::RayIntersection"]]
};if (window.register_type_impls) {window.register_type_impls(type_impls);} else {window.pending_type_impls = type_impls;}})()