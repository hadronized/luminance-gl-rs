(function() {var implementors = {};
implementors["luminance"] = ["impl&lt;C, T&gt; <a class='trait' href='https://doc.rust-lang.org/nightly/core/ops/trait.Drop.html' title='core::ops::Drop'>Drop</a> for <a class='struct' href='luminance/buffer/struct.Buffer.html' title='luminance::buffer::Buffer'>Buffer</a>&lt;C, T&gt; <span class='where'>where C: <a class='trait' href='luminance/buffer/trait.HasBuffer.html' title='luminance::buffer::HasBuffer'>HasBuffer</a></span>","impl&lt;C, L, D, CS, DS&gt; <a class='trait' href='https://doc.rust-lang.org/nightly/core/ops/trait.Drop.html' title='core::ops::Drop'>Drop</a> for <a class='struct' href='luminance/framebuffer/struct.Framebuffer.html' title='luminance::framebuffer::Framebuffer'>Framebuffer</a>&lt;C, L, D, CS, DS&gt; <span class='where'>where C: <a class='trait' href='luminance/texture/trait.HasTexture.html' title='luminance::texture::HasTexture'>HasTexture</a> + <a class='trait' href='luminance/framebuffer/trait.HasFramebuffer.html' title='luminance::framebuffer::HasFramebuffer'>HasFramebuffer</a>, L: <a class='trait' href='luminance/texture/trait.Layerable.html' title='luminance::texture::Layerable'>Layerable</a>, D: <a class='trait' href='luminance/texture/trait.Dimensionable.html' title='luminance::texture::Dimensionable'>Dimensionable</a>, D::Size: <a class='trait' href='https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html' title='core::marker::Copy'>Copy</a>, CS: <a class='trait' href='luminance/framebuffer/trait.ColorSlot.html' title='luminance::framebuffer::ColorSlot'>ColorSlot</a>&lt;C, L, D&gt;, DS: <a class='trait' href='luminance/framebuffer/trait.DepthSlot.html' title='luminance::framebuffer::DepthSlot'>DepthSlot</a>&lt;C, L, D&gt;</span>","impl&lt;C, T&gt; <a class='trait' href='https://doc.rust-lang.org/nightly/core/ops/trait.Drop.html' title='core::ops::Drop'>Drop</a> for <a class='struct' href='luminance/shader/program/struct.Program.html' title='luminance::shader::program::Program'>Program</a>&lt;C, T&gt; <span class='where'>where C: <a class='trait' href='luminance/shader/program/trait.HasProgram.html' title='luminance::shader::program::HasProgram'>HasProgram</a></span>","impl&lt;C, T&gt; <a class='trait' href='https://doc.rust-lang.org/nightly/core/ops/trait.Drop.html' title='core::ops::Drop'>Drop</a> for <a class='struct' href='luminance/shader/stage/struct.Stage.html' title='luminance::shader::stage::Stage'>Stage</a>&lt;C, T&gt; <span class='where'>where C: <a class='trait' href='luminance/shader/stage/trait.HasStage.html' title='luminance::shader::stage::HasStage'>HasStage</a></span>","impl&lt;C&gt; <a class='trait' href='https://doc.rust-lang.org/nightly/core/ops/trait.Drop.html' title='core::ops::Drop'>Drop</a> for <a class='struct' href='luminance/tessellation/struct.Tessellation.html' title='luminance::tessellation::Tessellation'>Tessellation</a>&lt;C&gt; <span class='where'>where C: <a class='trait' href='luminance/tessellation/trait.HasTessellation.html' title='luminance::tessellation::HasTessellation'>HasTessellation</a></span>","impl&lt;C, L, D, P&gt; <a class='trait' href='https://doc.rust-lang.org/nightly/core/ops/trait.Drop.html' title='core::ops::Drop'>Drop</a> for <a class='struct' href='luminance/texture/struct.Texture.html' title='luminance::texture::Texture'>Texture</a>&lt;C, L, D, P&gt; <span class='where'>where C: <a class='trait' href='luminance/texture/trait.HasTexture.html' title='luminance::texture::HasTexture'>HasTexture</a>, L: <a class='trait' href='luminance/texture/trait.Layerable.html' title='luminance::texture::Layerable'>Layerable</a>, D: <a class='trait' href='luminance/texture/trait.Dimensionable.html' title='luminance::texture::Dimensionable'>Dimensionable</a>, P: <a class='trait' href='luminance/pixel/trait.Pixel.html' title='luminance::pixel::Pixel'>Pixel</a></span>",];implementors["gl"] = ["impl&lt;T&gt; <a class='trait' href='https://doc.rust-lang.org/nightly/core/ops/trait.Drop.html' title='core::ops::Drop'>Drop</a> for <a class='struct' href='https://doc.rust-lang.org/nightly/collections/vec_deque/struct.VecDeque.html' title='collections::vec_deque::VecDeque'>VecDeque</a>&lt;T&gt;","impl&lt;'a, T&gt; <a class='trait' href='https://doc.rust-lang.org/nightly/core/ops/trait.Drop.html' title='core::ops::Drop'>Drop</a> for <a class='struct' href='https://doc.rust-lang.org/nightly/collections/vec_deque/struct.Drain.html' title='collections::vec_deque::Drain'>Drain</a>&lt;'a, T&gt; <span class='where'>where T: 'a</span>","impl&lt;T&gt; <a class='trait' href='https://doc.rust-lang.org/nightly/core/ops/trait.Drop.html' title='core::ops::Drop'>Drop</a> for <a class='struct' href='https://doc.rust-lang.org/nightly/collections/linked_list/struct.LinkedList.html' title='collections::linked_list::LinkedList'>LinkedList</a>&lt;T&gt;","impl&lt;W&gt; <a class='trait' href='https://doc.rust-lang.org/nightly/core/ops/trait.Drop.html' title='core::ops::Drop'>Drop</a> for <a class='struct' href='https://doc.rust-lang.org/nightly/std/io/buffered/struct.BufWriter.html' title='std::io::buffered::BufWriter'>BufWriter</a>&lt;W&gt; <span class='where'>where W: <a class='trait' href='https://doc.rust-lang.org/nightly/std/io/trait.Write.html' title='std::io::Write'>Write</a></span>","impl <a class='trait' href='https://doc.rust-lang.org/nightly/core/ops/trait.Drop.html' title='core::ops::Drop'>Drop</a> for <a class='struct' href='https://doc.rust-lang.org/nightly/std/sync/mpsc/select/struct.Select.html' title='std::sync::mpsc::select::Select'>Select</a>","impl&lt;'rx, T&gt; <a class='trait' href='https://doc.rust-lang.org/nightly/core/ops/trait.Drop.html' title='core::ops::Drop'>Drop</a> for <a class='struct' href='https://doc.rust-lang.org/nightly/std/sync/mpsc/select/struct.Handle.html' title='std::sync::mpsc::select::Handle'>Handle</a>&lt;'rx, T&gt; <span class='where'>where T: <a class='trait' href='https://doc.rust-lang.org/nightly/core/marker/trait.Send.html' title='core::marker::Send'>Send</a></span>","impl&lt;T&gt; <a class='trait' href='https://doc.rust-lang.org/nightly/core/ops/trait.Drop.html' title='core::ops::Drop'>Drop</a> for <a class='struct' href='https://doc.rust-lang.org/nightly/std/sync/mpsc/struct.Sender.html' title='std::sync::mpsc::Sender'>Sender</a>&lt;T&gt;","impl&lt;T&gt; <a class='trait' href='https://doc.rust-lang.org/nightly/core/ops/trait.Drop.html' title='core::ops::Drop'>Drop</a> for <a class='struct' href='https://doc.rust-lang.org/nightly/std/sync/mpsc/struct.SyncSender.html' title='std::sync::mpsc::SyncSender'>SyncSender</a>&lt;T&gt;","impl&lt;T&gt; <a class='trait' href='https://doc.rust-lang.org/nightly/core/ops/trait.Drop.html' title='core::ops::Drop'>Drop</a> for <a class='struct' href='https://doc.rust-lang.org/nightly/std/sync/mpsc/struct.Receiver.html' title='std::sync::mpsc::Receiver'>Receiver</a>&lt;T&gt;","impl <a class='trait' href='https://doc.rust-lang.org/nightly/core/ops/trait.Drop.html' title='core::ops::Drop'>Drop</a> for <a class='struct' href='https://doc.rust-lang.org/nightly/std/sync/condvar/struct.Condvar.html' title='std::sync::condvar::Condvar'>Condvar</a>","impl&lt;T&gt; <a class='trait' href='https://doc.rust-lang.org/nightly/core/ops/trait.Drop.html' title='core::ops::Drop'>Drop</a> for <a class='struct' href='https://doc.rust-lang.org/nightly/std/sync/mutex/struct.Mutex.html' title='std::sync::mutex::Mutex'>Mutex</a>&lt;T&gt; <span class='where'>where T: ?<a class='trait' href='https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html' title='core::marker::Sized'>Sized</a></span>","impl&lt;'a, T&gt; <a class='trait' href='https://doc.rust-lang.org/nightly/core/ops/trait.Drop.html' title='core::ops::Drop'>Drop</a> for <a class='struct' href='https://doc.rust-lang.org/nightly/std/sync/mutex/struct.MutexGuard.html' title='std::sync::mutex::MutexGuard'>MutexGuard</a>&lt;'a, T&gt; <span class='where'>where T: ?<a class='trait' href='https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html' title='core::marker::Sized'>Sized</a></span>","impl&lt;T&gt; <a class='trait' href='https://doc.rust-lang.org/nightly/core/ops/trait.Drop.html' title='core::ops::Drop'>Drop</a> for <a class='struct' href='https://doc.rust-lang.org/nightly/std/sync/rwlock/struct.RwLock.html' title='std::sync::rwlock::RwLock'>RwLock</a>&lt;T&gt; <span class='where'>where T: ?<a class='trait' href='https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html' title='core::marker::Sized'>Sized</a></span>","impl&lt;'a, T&gt; <a class='trait' href='https://doc.rust-lang.org/nightly/core/ops/trait.Drop.html' title='core::ops::Drop'>Drop</a> for <a class='struct' href='https://doc.rust-lang.org/nightly/std/sync/rwlock/struct.RwLockReadGuard.html' title='std::sync::rwlock::RwLockReadGuard'>RwLockReadGuard</a>&lt;'a, T&gt; <span class='where'>where T: ?<a class='trait' href='https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html' title='core::marker::Sized'>Sized</a></span>","impl&lt;'a, T&gt; <a class='trait' href='https://doc.rust-lang.org/nightly/core/ops/trait.Drop.html' title='core::ops::Drop'>Drop</a> for <a class='struct' href='https://doc.rust-lang.org/nightly/std/sync/rwlock/struct.RwLockWriteGuard.html' title='std::sync::rwlock::RwLockWriteGuard'>RwLockWriteGuard</a>&lt;'a, T&gt; <span class='where'>where T: ?<a class='trait' href='https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html' title='core::marker::Sized'>Sized</a></span>","impl&lt;T&gt; <a class='trait' href='https://doc.rust-lang.org/nightly/core/ops/trait.Drop.html' title='core::ops::Drop'>Drop</a> for <a class='struct' href='https://doc.rust-lang.org/nightly/alloc/boxed/struct.IntermediateBox.html' title='alloc::boxed::IntermediateBox'>IntermediateBox</a>&lt;T&gt; <span class='where'>where T: ?<a class='trait' href='https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html' title='core::marker::Sized'>Sized</a></span>","impl&lt;'a&gt; <a class='trait' href='https://doc.rust-lang.org/nightly/core/ops/trait.Drop.html' title='core::ops::Drop'>Drop</a> for <a class='struct' href='https://doc.rust-lang.org/nightly/collections/string/struct.Drain.html' title='collections::string::Drain'>Drain</a>&lt;'a&gt;","impl&lt;T&gt; <a class='trait' href='https://doc.rust-lang.org/nightly/core/ops/trait.Drop.html' title='core::ops::Drop'>Drop</a> for <a class='struct' href='https://doc.rust-lang.org/nightly/alloc/rc/struct.Rc.html' title='alloc::rc::Rc'>Rc</a>&lt;T&gt; <span class='where'>where T: ?<a class='trait' href='https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html' title='core::marker::Sized'>Sized</a></span>","impl&lt;T&gt; <a class='trait' href='https://doc.rust-lang.org/nightly/core/ops/trait.Drop.html' title='core::ops::Drop'>Drop</a> for <a class='struct' href='https://doc.rust-lang.org/nightly/alloc/rc/struct.Weak.html' title='alloc::rc::Weak'>Weak</a>&lt;T&gt; <span class='where'>where T: ?<a class='trait' href='https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html' title='core::marker::Sized'>Sized</a></span>","impl&lt;T&gt; <a class='trait' href='https://doc.rust-lang.org/nightly/core/ops/trait.Drop.html' title='core::ops::Drop'>Drop</a> for <a class='struct' href='https://doc.rust-lang.org/nightly/collections/vec/struct.Vec.html' title='collections::vec::Vec'>Vec</a>&lt;T&gt;","impl&lt;T&gt; <a class='trait' href='https://doc.rust-lang.org/nightly/core/ops/trait.Drop.html' title='core::ops::Drop'>Drop</a> for <a class='struct' href='https://doc.rust-lang.org/nightly/collections/vec/struct.IntoIter.html' title='collections::vec::IntoIter'>IntoIter</a>&lt;T&gt;","impl&lt;'a, T&gt; <a class='trait' href='https://doc.rust-lang.org/nightly/core/ops/trait.Drop.html' title='core::ops::Drop'>Drop</a> for <a class='struct' href='https://doc.rust-lang.org/nightly/collections/vec/struct.Drain.html' title='collections::vec::Drain'>Drain</a>&lt;'a, T&gt;",];implementors["luminance_gl"] = ["impl&lt;C, T&gt; <a class='trait' href='https://doc.rust-lang.org/nightly/core/ops/trait.Drop.html' title='core::ops::Drop'>Drop</a> for <a class='struct' href='luminance/buffer/struct.Buffer.html' title='luminance::buffer::Buffer'>Buffer</a>&lt;C, T&gt; <span class='where'>where C: <a class='trait' href='luminance/buffer/trait.HasBuffer.html' title='luminance::buffer::HasBuffer'>HasBuffer</a></span>","impl&lt;C, L, D, CS, DS&gt; <a class='trait' href='https://doc.rust-lang.org/nightly/core/ops/trait.Drop.html' title='core::ops::Drop'>Drop</a> for <a class='struct' href='luminance/framebuffer/struct.Framebuffer.html' title='luminance::framebuffer::Framebuffer'>Framebuffer</a>&lt;C, L, D, CS, DS&gt; <span class='where'>where C: <a class='trait' href='luminance/texture/trait.HasTexture.html' title='luminance::texture::HasTexture'>HasTexture</a> + <a class='trait' href='luminance/framebuffer/trait.HasFramebuffer.html' title='luminance::framebuffer::HasFramebuffer'>HasFramebuffer</a>, CS: <a class='trait' href='luminance/framebuffer/trait.ColorSlot.html' title='luminance::framebuffer::ColorSlot'>ColorSlot</a>&lt;C, L, D&gt;, D: <a class='trait' href='luminance/texture/trait.Dimensionable.html' title='luminance::texture::Dimensionable'>Dimensionable</a>, DS: <a class='trait' href='luminance/framebuffer/trait.DepthSlot.html' title='luminance::framebuffer::DepthSlot'>DepthSlot</a>&lt;C, L, D&gt;, L: <a class='trait' href='luminance/texture/trait.Layerable.html' title='luminance::texture::Layerable'>Layerable</a>, D::<a class='trait' href='luminance/texture/trait.Dimensionable.html' title='luminance::texture::Dimensionable'>Size</a>: <a class='trait' href='https://doc.rust-lang.org/nightly/core/marker/trait.Copy.html' title='core::marker::Copy'>Copy</a></span>","impl&lt;C, T&gt; <a class='trait' href='https://doc.rust-lang.org/nightly/core/ops/trait.Drop.html' title='core::ops::Drop'>Drop</a> for <a class='struct' href='luminance/shader/program/struct.Program.html' title='luminance::shader::program::Program'>Program</a>&lt;C, T&gt; <span class='where'>where C: <a class='trait' href='luminance/shader/program/trait.HasProgram.html' title='luminance::shader::program::HasProgram'>HasProgram</a></span>","impl&lt;C, T&gt; <a class='trait' href='https://doc.rust-lang.org/nightly/core/ops/trait.Drop.html' title='core::ops::Drop'>Drop</a> for <a class='struct' href='luminance/shader/stage/struct.Stage.html' title='luminance::shader::stage::Stage'>Stage</a>&lt;C, T&gt; <span class='where'>where C: <a class='trait' href='luminance/shader/stage/trait.HasStage.html' title='luminance::shader::stage::HasStage'>HasStage</a></span>","impl&lt;C&gt; <a class='trait' href='https://doc.rust-lang.org/nightly/core/ops/trait.Drop.html' title='core::ops::Drop'>Drop</a> for <a class='struct' href='luminance/tessellation/struct.Tessellation.html' title='luminance::tessellation::Tessellation'>Tessellation</a>&lt;C&gt; <span class='where'>where C: <a class='trait' href='luminance/tessellation/trait.HasTessellation.html' title='luminance::tessellation::HasTessellation'>HasTessellation</a></span>","impl&lt;C, L, D, P&gt; <a class='trait' href='https://doc.rust-lang.org/nightly/core/ops/trait.Drop.html' title='core::ops::Drop'>Drop</a> for <a class='struct' href='luminance/texture/struct.Texture.html' title='luminance::texture::Texture'>Texture</a>&lt;C, L, D, P&gt; <span class='where'>where C: <a class='trait' href='luminance/texture/trait.HasTexture.html' title='luminance::texture::HasTexture'>HasTexture</a>, D: <a class='trait' href='luminance/texture/trait.Dimensionable.html' title='luminance::texture::Dimensionable'>Dimensionable</a>, L: <a class='trait' href='luminance/texture/trait.Layerable.html' title='luminance::texture::Layerable'>Layerable</a>, P: <a class='trait' href='luminance/pixel/trait.Pixel.html' title='luminance::pixel::Pixel'>Pixel</a></span>",];

            if (window.register_implementors) {
                window.register_implementors(implementors);
            } else {
                window.pending_implementors = implementors;
            }
        
})()