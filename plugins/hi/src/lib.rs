use kovi::PluginBuilder as plugin;
use kovi_onebot::EventRegistrar;

#[kovi::plugin]
async fn main() {
    plugin::on_admin_msg(|event| async move {
        if event.borrow_text() == Some("hi") {
            event.reply("hi")
        }
    });

    plugin::on_admin_msg(async move |event| {
        if event.borrow_text() == Some("hello") {
            event.reply("kovi")
        }
    });
}
