import * as fs from "fs";

import { getSigningClient, getQueryClient } from "./util";

const run = async () => {
  const { threadContractAddress } = JSON.parse(
    fs.readFileSync("scripts/contract_addresses.json").toString()
  );
  const { signerAddress, siggingClient } = await getSigningClient();
  const queryClient = await getQueryClient();

  const asker_user_id = "1";
  const answerer_user_id = "2";
  const new_thread = {
    title: "title aaa",
    labels: ["label1", "label2"],
    mutable: false,
  };
  const ask_in_new_thread_msg = {
    new_thread,
    fee_mode: "fixed",
    answerer_user_id,
    content_storage: {
      on_chain: {
        content: "hahaha",
        content_format: "plain_text",
      },
    },
    tip: "100",
  };
  const cost = await queryClient.queryContractSmart(threadContractAddress, {
    query_cost_to_ask_in_new_thread: {
      asker_user_id,
      ask_in_new_thread_msg,
    },
  });
  console.log("cost", JSON.stringify(cost, null, 2));
  await siggingClient
    .execute(
      signerAddress,
      threadContractAddress,
      {
        ask_in_new_thread: ask_in_new_thread_msg,
      },
      "auto",
      "memooooo",
      [
        {
          denom: "uosmo",
          amount: cost.total_needed_from_user as string,
        },
      ]
    )
    .then((res) => {
      console.log(res.transactionHash);
    });
};

run();

// factory/neutron1xdtwh5jr4zjx8g3zh29jud75c666wua7tsmum3ajm6ylf782etfs60dj2h/wstETH
// {
//   "query_denom": {
//     "denom": "factory/neutron1xdtwh5jr4zjx8g3zh29jud75c666wua7tsmum3ajm6ylf782etfs60dj2h/wstETH"
//   }
// }