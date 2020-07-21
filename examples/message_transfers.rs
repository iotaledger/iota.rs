//! Send a transfer to an address.
//!
//! Run with:
//!
//! ```
//! cargo run --example message_transfers
//! ```
use anyhow::Result;
use iota::{
    transaction::bundled::{Address, Tag, BundledTransactionField},
    client::Transfer,
    ternary::TryteBuf,
};
use iota_conversion::Trinary;

#[smol_potat::main]
async fn main() -> Result<()> {
    // Prepare a vector of transfers
    let mut transfers = Vec::new();

    // Push the transfer to vector.
    transfers.push(Transfer {
        // Address is 81 trytes.
        address: Address::from_inner_unchecked(
            TryteBuf::try_from_str(
                "RVORZ9SIIP9RCYMREUIXXVPQIPHVCNPQ9HZWYKFWYWZRE9JQKG9REPKIASHUUECPSQO9JT9XNMVKWYGVA",
            )
            .unwrap()
            .as_trits()
            .encode(),
        ),
        // We are using a zero balance seed so we make a zero value transfer here
        value: 0,
        message: Some(String::from("

        Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nulla dapibus eros at tincidunt fringilla. Vestibulum ante ipsum primis in faucibus orci luctus et ultrices posuere cubilia curae; Sed mattis orci felis, vel pulvinar est sodales non. Vestibulum placerat luctus dapibus. Quisque finibus lacus a mauris fermentum eleifend. Maecenas pharetra justo nunc, in egestas magna suscipit at. Vivamus dictum at est pharetra porttitor. Integer ac neque elementum, feugiat lorem a, suscipit nisi. Etiam vel vehicula ligula. In bibendum aliquet vulputate. Pellentesque tincidunt elementum convallis.
        
        Maecenas auctor erat quis gravida pellentesque. Nam posuere gravida nisl. Ut tincidunt vitae neque placerat varius. Aliquam eget ultrices sem. Sed iaculis convallis magna at porttitor. Ut blandit ut ipsum id pellentesque. Vestibulum a egestas nisi, at auctor arcu. Nulla bibendum ut magna nec elementum. In sed ipsum nec nunc tincidunt porttitor. Sed sed aliquam enim. Sed ut elementum sapien, sit amet porta magna. Donec eget nulla nec sem sodales mollis at vel magna. In rhoncus ornare lectus a aliquet.
        
        Curabitur sodales elit eget leo efficitur cursus. Proin eget neque dictum, luctus ipsum sed, porta ex. Morbi eget libero sem. Etiam vitae dignissim nunc. In elementum eu augue eget malesuada. Suspendisse at ornare erat, et convallis elit. Suspendisse rhoncus metus odio, sed ornare quam consectetur ut. Suspendisse a nunc vel leo iaculis varius. Aliquam erat volutpat. Nullam semper vulputate odio, et sagittis nunc ultrices non.
        
        Curabitur sit amet sem laoreet, egestas magna in, eleifend orci. Maecenas tincidunt nunc a eros sollicitudin, quis gravida tellus interdum. Curabitur id ipsum diam. Proin at massa urna. Sed neque mauris, efficitur vitae congue nec, posuere a magna. Vestibulum nunc tortor, euismod non faucibus a, aliquet eget sapien. Vivamus eros dui, viverra at mauris a, cursus pulvinar turpis. Vivamus non tempor leo. Donec condimentum nisl a turpis aliquam aliquet. Phasellus sollicitudin ultricies orci, ac ultrices leo. Praesent ultricies, quam accumsan ornare sodales, massa lacus tempus nunc, sed convallis metus dui et nibh.
        
        Aenean ultrices porttitor enim. Vestibulum vel ante at neque laoreet sollicitudin ut nec purus. Donec tortor lectus, egestas non gravida nec, consequat eu orci. Sed id vehicula eros. Aliquam leo odio, finibus nec placerat sit amet, dignissim in est. Suspendisse in neque auctor, bibendum nunc non, luctus dolor. Nulla convallis felis ac libero egestas, at egestas nunc consequat. Suspendisse tellus massa, consequat non felis facilisis, maximus convallis tortor. Fusce porta nunc est, et facilisis dolor imperdiet vitae. Vivamus ullamcorper erat et dignissim mattis. Nulla maximus, nisi vel lobortis gravida, arcu neque blandit ante, at sodales risus enim a metus. Nulla quis finibus mauris. ")),
        tag: Some(Tag::try_from_inner(TryteBuf::try_from_str("LOREMIPSUM99999999999999999")
        .unwrap()
        .as_trits()
        .encode()).unwrap()),
    });

    // Create a client instance
    iota::Client::add_node("https://nodes.comnet.thetangle.org")?;
    // Call send_transfers api
    // Below is just a dummy seed which just serves as an example.
    // If you want to replace your own. It probably should be a seed with balance on comnet/devnet.
    let res = iota::Client::send(None)
        // Input the transfers
        .transfers(transfers)
        // We are sending to comnet, so mwm should be 10. It's 14 by default if you don't call this.
        .min_weight_magnitude(10)
        // Sending to the node and receive the response
        .send()
        .await?;

    // The response of send_transfers is vector of Transaction type. We choose the first one and see what is its bundle hash
    println!("{:?}", res[0].bundle().to_inner().as_i8_slice().trytes());

    Ok(())
}
