import { Injectable } from '@nestjs/common';
import chainweb from '@kadena/chainwebjs';
import { IBlockHeader, ICutResponse } from '@kadena/chainwebjs/lib/types';
import { createSignWithKeypair, Pact } from '@kadena/client';

const API = {
  network: 'testnet04',
  host: 'https://api.testnet.chainweb.com',
} as const;

@Injectable()
export class AppService {
  async getCurrentCut(): Promise<ICutResponse> {
    return chainweb.cut.current(API.network, API.host);
  }

  async getBlockHeader(): Promise<IBlockHeader[]> {
    return chainweb.header.recent(1, 100, undefined, API.network, API.host);
  }

  async buildTx() {
    const builder = Pact.builder
      .execution('(format "Hello {}!" [(read-msg "person")])')
      .addSigner(
        '2FAC76CF704FBB1E0FBA6B75685A9F67BB757472D86E454BFEC3D83710AB64F0',
      )
      .setMeta({
        chainId: '8',
        senderAccount:
          '2FAC76CF704FBB1E0FBA6B75685A9F67BB757472D86E454BFEC3D83710AB64F0',
      })
      .setNetworkId(API.network)
      .createTransaction();

    const signer = createSignWithKeypair({
      publicKey:
        '2FAC76CF704FBB1E0FBA6B75685A9F67BB757472D86E454BFEC3D83710AB64F0',
      secretKey:
        '8030E7DBD31C5EA04E5E2A03A5AF132F5890DF4BBC9DA12992B5D9F52E15A2AA',
    });

    return signer(builder);
  }
}
