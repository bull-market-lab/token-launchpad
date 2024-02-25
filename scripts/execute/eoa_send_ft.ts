import { getSigningClient } from "../util";

const run = async () => {
  const { signerAddress, siggingClient } = await getSigningClient();

  const sendAmount = 700;
  const recipientAddress = "neutron1d5k3kzd98683zcg6p8ev8h56tg6tk83pry02xx";

  const denom =
    "factory/neutron1fvfek4nxjrk807d5aq5p5jyg4qarzlef03u2v9eyryrwkjkwnwss95wz6x/ubad404";

  await siggingClient
    .sendTokens(
      signerAddress,
      recipientAddress,
      [
        {
          denom,
          amount: sendAmount.toString(),
        },
      ],
      "auto",
      "memooooo"
    )
    .then((res) => {
      console.log(res.transactionHash);
    });
};

run();
