use console_error_panic_hook::set_once as set_panic_hook;
use log::{info, trace, warn};
use wasm_bindgen::JsValue;
use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::{ErrorEvent, MessageEvent, WebSocket};
use yew::{html, Component, Context, Html};

// All actions
enum Msg {
    AddOne,
    GetStuff,
    GotMessage,
    Ping,
    WsMessage(String),
}

// global model
struct Model {
    text: String,
    ws_client: WebSocket,
}

// yew-component
impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let ws_client = WebSocket::new("ws://localhost:3000/ws").unwrap();
        ws_client.set_binary_type(web_sys::BinaryType::Arraybuffer);

        ctx.link()
            .send_message(Msg::WsMessage("Created".to_string()));
        info!("ready_state: {:?}", ws_client.ready_state());
        let cloned_ws = ws_client.clone();

        let bla = ctx.link().clone();

        let onmessage_callback = Closure::wrap(Box::new(move |e: MessageEvent| {
            if let Ok(abuf) = e.data().dyn_into::<js_sys::ArrayBuffer>() {
                info!("message event, received arraybuffer: {:?}", abuf);
                let array = js_sys::Uint8Array::new(&abuf);
                let len = array.byte_length() as usize;
                info!("Arraybuffer received {}bytes: {:?}", len, array.to_vec());
                cloned_ws.set_binary_type(web_sys::BinaryType::Blob);

                match cloned_ws.send_with_u8_array(&vec![5, 6, 7, 8]) {
                    Ok(_) => info!("binary message successfully sent"),
                    Err(err) => info!("error sending message: {:?}", err),
                }
            } else if let Ok(blob) = e.data().dyn_into::<web_sys::Blob>() {
                info!("message event, received blob: {:?}", blob);
                let fr = web_sys::FileReader::new().unwrap();
                let fr_c = fr.clone();
                let onloadend_cb = Closure::wrap(Box::new(move |_e: web_sys::ProgressEvent| {
                    let array = js_sys::Uint8Array::new(&fr_c.result().unwrap());
                    let len = array.byte_length() as usize;
                    info!("Blob received {}bytes: {:?}", len, array.to_vec());
                })
                    as Box<dyn FnMut(web_sys::ProgressEvent)>);
                fr.set_onloadend(Some(onloadend_cb.as_ref().unchecked_ref()));
                fr.read_as_array_buffer(&blob).expect("blob not readable");
                onloadend_cb.forget();
            } else if let Ok(txt) = e.data().dyn_into::<js_sys::JsString>() {
                //ctx.link().send_message(Msg::GotMessage);
                let x: String = txt.into();

                bla.send_message(Msg::WsMessage(x));
                //info!("message event, received Text: {:?}", move x);
            } else {
                info!("message event, received Unknown: {:?}", e.data());
            }
        }) as Box<dyn FnMut(MessageEvent)>);
        // set message event handler on WebSocket
        ws_client.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
        // forget the callback to keep it alive
        onmessage_callback.forget();

        let onerror_callback = Closure::wrap(Box::new(move |e: ErrorEvent| {
            info!("error event: {:?}", e);
        }) as Box<dyn FnMut(ErrorEvent)>);
        ws_client.set_onerror(Some(onerror_callback.as_ref().unchecked_ref()));
        onerror_callback.forget();

        let cloned_ws = ws_client.clone();
        let onopen_callback = Closure::wrap(Box::new(move |_| {
            info!("socket opened");
            match cloned_ws.send_with_str("ping") {
                Ok(_) => info!("str message successfully sent"),
                Err(err) => warn!("error sending message: {:?}", err),
            }
            // send off binary message
            match cloned_ws.send_with_u8_array(&vec![0, 1, 2, 3]) {
                Ok(_) => info!("binary message successfully sent"),
                Err(err) => warn!("error sending message: {:?}", err),
            }
        }) as Box<dyn FnMut(JsValue)>);

        ws_client.set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));
        onopen_callback.forget();

        //ws_client.send_with_str("data").unwrap();
        //let bla = ws_client.onopen().unwrap();

        //let res = bla.apply(ctx, []);

        Self {
            text: "nothing".to_string(),
            ws_client: ws_client,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddOne => {
                self.text = "clicked".to_string();
                true
            }
            Msg::GetStuff => {
                self.text = "GetStuff".to_string();
                true
            }
            Msg::WsMessage(s) => {
                self.text = s;
                true
            }
            Msg::GotMessage => {
                self.text = "GotMessage".to_string();
                true
            }
            Msg::Ping => {
                self.ws_client.send_with_str("data").unwrap();
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        let link = ctx.link();
        html! {
            <div>
                <button onclick={link.callback(|_| Msg::AddOne)}>{ "click" }</button>
                <button onclick={link.callback(|_| Msg::GetStuff)}>{ "stuff" }</button>
                <button onclick={link.callback(|_| Msg::Ping)}>{ "ping" }</button>
                <p>{ self.text.to_ascii_lowercase() }</p>
            </div>
        }
    }
}

fn main() {
    set_panic_hook();
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<Model>();
}
