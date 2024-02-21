import { getSigningClient } from "../util";

const run = async () => {
  const { signerAddress, siggingClient } = await getSigningClient();

  const sendAmount = 500_000;
  const recipientAddress = "neutron1d5k3kzd98683zcg6p8ev8h56tg6tk83pry02xx";

  const denom =
    "factory/neutron1y2ddy0e8kpdd9hg6dj5rztawyhf45vlptgegkegam40hth2y8azs6dhnrq/ubad404";

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
