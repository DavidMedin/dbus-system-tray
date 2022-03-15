use zbus::{Connection,MessageHeader,Result,dbus_proxy,dbus_interface,
    zvariant::ObjectPath};


#[dbus_proxy(
    interface = "org.kde.StatusNotifierItem"
)]
trait Item {
    #[dbus_proxy(property)]
    fn category(&self) -> Result<String>;
    #[dbus_proxy(property)]
    fn id(&self) -> Result<String>;
    #[dbus_proxy(property)]
    fn title(&self) -> Result<String>;
    #[dbus_proxy(property)]
    fn status(&self) -> Result<String>;
    #[dbus_proxy(property)]
    fn window_id(&self) -> Result<u32>;
    #[dbus_proxy(property)]
    fn icon_name(&self) -> Result<String>;
    #[dbus_proxy(property)]
    fn icon_pixmap(&self) -> Result<(i32,i32,Vec<u8>)>;
    #[dbus_proxy(property)]
    fn overlay_icon_name(&self) -> Result<String>;
    #[dbus_proxy(property)]
    fn overlay_icon_pixmap(&self) -> Result<(i32,i32,Vec<u8>)>;
    #[dbus_proxy(property)]
    fn attention_icon_name(&self) -> Result<String>;
    #[dbus_proxy(property)]
    fn attention_icon_pixmap(&self) -> Result<(i32,i32,Vec<u8>)>;
    #[dbus_proxy(property)]
    fn attention_movie_name(&self) -> Result<String>;
    #[dbus_proxy(property)]
    fn tool_tip(&self) -> Result<(String,Vec<(i32,i32,Vec<u8>)>,String,String)>;
    #[dbus_proxy(property)]
    fn item_is_menu(&self) -> Result<bool>;
    #[dbus_proxy(property)]
    fn menu(&self) -> Result<ObjectPath<'_>>;

    // Methods
    fn context_menu(&self, x: i32, y: i32) -> Result<()>;
    fn activate(&self, x: i32, y : i32) -> Result<()>;
    fn secondary_activate(&self, x : i32, y : i32) -> Result<()>;
    fn scroll(&self, delta : i32, orientation: String) -> Result<()>;

    //Signals
    #[dbus_proxy(signal)]
    fn new_title(&self) -> Result<()>;
    #[dbus_proxy(signal)]
    fn new_icon(&self) -> Result<()>;
    #[dbus_proxy(signal)]
    fn new_attention_icon(&self) -> Result<()>;
    #[dbus_proxy(signal)]
    fn new_overlay_icon(&self) -> Result<()>;
    #[dbus_proxy(signal)]
    fn new_tool_tip(&self) -> Result<()>;
    #[dbus_proxy(signal)]
    fn new_status(&self) -> Result<()>;
}

struct StatusNotifierWatcher{}
#[dbus_interface(name = "org.kde.StatusNotifierWatcher")]
impl StatusNotifierWatcher {
   async fn register_status_notifier_item(&mut self, service: &str,
                                          #[zbus(header)] header: MessageHeader<'_>,
                                          #[zbus(connection)] conn : &Connection)
   {
       //Get the caller of this method
       let sender : &str = if let Some(author) = header.sender().unwrap() {
            author.as_str()
       }else { panic!(); };

       // Get some arbitrary property of this object.
       let caller = ItemProxy::builder(conn)
           .destination(sender).unwrap()
           .path(service).unwrap().build().await.unwrap();

        let title = caller.title().await.unwrap();
       println!("Hello, {}, from {}. {} is your title!", service,sender,title);
   }
   fn register_status_notifier_host(&mut self, service: &str){
       println!("Hello host, {}",service);
   }
}


#[async_std::main]
async fn main() -> std::result::Result<(),Box<dyn std::error::Error>>{
    let watcher = StatusNotifierWatcher{};
    let connection = Connection::session().await?;
    connection.object_server().at("/StatusNotifierWatcher",watcher).await?;
    connection.request_name("org.kde.StatusNotifierWatcher").await?;

    loop {
        std::thread::park();
    }
}