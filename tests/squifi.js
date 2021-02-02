const anchor = require('@project-serum/anchor');

describe('squifi', () => {

  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.local());

  it('Is initialized!', async () => {
    // Add your test here.
    const program = anchor.workspace.Squifi;
    const tx = await program.rpc.initialize();
    console.log("Your transaction signature", tx);
  });
});
