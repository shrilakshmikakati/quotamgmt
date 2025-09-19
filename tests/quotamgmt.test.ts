import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { expect } from "chai";
import { Keypair, PublicKey, SystemProgram } from "@solana/web3.js";

// Import the correct type for the program from the generated types.
// This is essential for proper type-checking.
import type { QuotaManagement } from "../target/types/quota_management";

// Try different possible import patterns
let QuotaProgram: any;
try {
  // Try the snake_case version
  QuotaProgram = require("../target/types/quota_management");
} catch {
  try {
    // Try the original quotamanagement version
    QuotaProgram = require("../target/types/quotamanagement");
  } catch {
    console.log("Could not find types file. Make sure to run 'anchor build' first.");
    process.exit(1);
  }
}

describe("quota-management", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  // Try to get the program from workspace and explicitly type it.
  // This resolves the 'Property does not exist' error.
  let program: Program<QuotaManagement>;
  try {
    program = anchor.workspace.QuotaManagement as Program<QuotaManagement>;
  } catch {
    try {
      program = anchor.workspace.Quotamanagement as Program<QuotaManagement>;
    } catch {
      try {
        program = anchor.workspace.quotamanagement as Program<QuotaManagement>;
      } catch (error) {
        console.error("Could not find program in workspace:", error);
        process.exit(1);
      }
    }
  }

  // Test accounts
  const regulator = Keypair.generate();
  const holder1 = Keypair.generate();
  const holder2 = Keypair.generate();

  // Test data
  const concessionId1 = "MINE001";
  const concessionId2 = "MINE002";
  const allocatedQuota = new anchor.BN(10000); // 10,000 tons
  const validityPeriod = new anchor.BN(Date.now() / 1000 + 365 * 24 * 60 * 60); // 1 year from now

  // PDAs
  let quotaAccount1: PublicKey;
  let quotaAccount2: PublicKey;
  let quotaBump1: number;
  let quotaBump2: number;

  before(async () => {
    console.log("Program ID:", program.programId.toString());

    // Airdrop SOL to test accounts
    await provider.connection.requestAirdrop(regulator.publicKey, 2 * anchor.web3.LAMPORTS_PER_SOL);
    await provider.connection.requestAirdrop(holder1.publicKey, 2 * anchor.web3.LAMPORTS_PER_SOL);
    await provider.connection.requestAirdrop(holder2.publicKey, 2 * anchor.web3.LAMPORTS_PER_SOL);

    // Find PDAs
    [quotaAccount1, quotaBump1] = PublicKey.findProgramAddressSync(
      [Buffer.from("quota"), Buffer.from(concessionId1), holder1.publicKey.toBuffer()],
      program.programId
    );

    [quotaAccount2, quotaBump2] = PublicKey.findProgramAddressSync(
      [Buffer.from("quota"), Buffer.from(concessionId2), holder2.publicKey.toBuffer()],
      program.programId
    );

    console.log("Quota Account 1:", quotaAccount1.toString());
    console.log("Quota Account 2:", quotaAccount2.toString());

    // Wait for airdrops to complete
    await new Promise(resolve => setTimeout(resolve, 2000));
  });

  describe("Initialize Quota", () => {
    it("Successfully initializes a quota", async () => {
      try {
        const tx = await program.methods
          .initializeQuota(
            concessionId1,
            allocatedQuota,
            validityPeriod,
            { annual: {} } // QuotaType::Annual
          )
          .accounts({
            quotaAccount: quotaAccount1,
            holder: holder1.publicKey,
            regulator: regulator.publicKey,
            systemProgram: SystemProgram.programId,
          })
          .signers([regulator])
          .rpc();

        console.log("Initialize quota transaction:", tx);

        // Fetch and verify the quota account
        const quotaData = await program.account.quotaAccount.fetch(quotaAccount1);

        expect(quotaData.concessionId).to.equal(concessionId1);
        expect(quotaData.holder.toString()).to.equal(holder1.publicKey.toString());
        expect(quotaData.regulator.toString()).to.equal(regulator.publicKey.toString());
        expect(quotaData.allocatedQuota.toString()).to.equal(allocatedQuota.toString());
        expect(quotaData.usedQuota.toString()).to.equal("0");
        expect(quotaData.availableQuota.toString()).to.equal(allocatedQuota.toString());

        console.log("Quota initialized successfully!");
      } catch (error) {
        console.error("Error initializing quota:", error);
        throw error;
      }
    });
  });

  describe("Use Quota", () => {
    const shipmentId = "SHIP001";
    const usageAmount = new anchor.BN(1000); // 1,000 tons
    let usageRecord: PublicKey;

    before(async () => {
      // Find usage record PDA
      [usageRecord] = PublicKey.findProgramAddressSync(
        [Buffer.from("usage"), Buffer.from(shipmentId), holder1.publicKey.toBuffer()],
        program.programId
      );
      console.log("Usage Record:", usageRecord.toString());
    });

    it("Successfully uses quota for a shipment", async () => {
      const qualityParams = {
        grossCalorificValue: 5500,
        moistureContent: 1200, // 12.00%
        ashContent: 1500, // 15.00%
        sulphurContent: 50, // 0.50%
        volatileMatter: 3500, // 35.00%
        fixedCarbon: 5000, // 50.00%
        coalGrade: { gradeB: {} },
        sizeClassification: "0-50mm",
      };

      try {
        await program.methods
          .useQuota(usageAmount, shipmentId, qualityParams)
          .accounts({
            quotaAccount: quotaAccount1,
            usageRecord: usageRecord,
            holder: holder1.publicKey,
            systemProgram: SystemProgram.programId,
          })
          .signers([holder1])
          .rpc();

        // Verify quota account updates
        const quotaData = await program.account.quotaAccount.fetch(quotaAccount1);
        expect(quotaData.usedQuota.toString()).to.equal(usageAmount.toString());
        expect(quotaData.availableQuota.toString()).to.equal(
          allocatedQuota.sub(usageAmount).toString()
        );

        // Verify usage record
        const usageData = await program.account.usageRecord.fetch(usageRecord);
        expect(usageData.concessionId).to.equal(concessionId1);
        expect(usageData.shipmentId).to.equal(shipmentId);
        expect(usageData.amount.toString()).to.equal(usageAmount.toString());

        console.log("Quota used successfully!");
      } catch (error) {
        console.error("Error using quota:", error);
        throw error;
      }
    });
  });
});
