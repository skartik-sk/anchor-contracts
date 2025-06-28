use anchor_lang::{prelude::*};
//use anchor_lang::system_program::{transfer,Transfer};
declare_id!("9ZjhPo2eP8jXiiw1DoDc4kNz3vxjk9ZKuUg9sE9CfpfA");


//useing sytem_progrm

// #[program]
// pub mod cpi_contract{
//     use super::*;

//     pub fn sol_trasfer(ctx: Context<SolTrasfer>, amount: u64) -> Result<()> {
//         let form_pub = ctx.accounts.sender.to_account_info();
//          let to_pub = ctx.accounts.recipient.to_account_info();
//          let program_id = ctx.accounts.system_program.to_account_info();

//          let cpicall = CpiContext::new(
//             program_id, 
//             Transfer{
//                 from:form_pub,
//                 to:to_pub,
                
//             }
        
//         );

//         transfer(cpicall, amount)?;

//         Ok(())
//     }


// }


#[program]
pub mod cpi_contract {
    use anchor_lang::solana_program::{instruction::Instruction, program::invoke};

    use super::*;

    pub fn sol_trasfer(ctx: Context<SolTrasfer>, amount: u64) -> Result<()> {
        let form_pub = ctx.accounts.sender.to_account_info();
        let to_pub = ctx.accounts.recipient.to_account_info();
        let program_id = ctx.accounts.system_program.to_account_info();

        let accout_meta = vec![
            AccountMeta::new(form_pub.key(), true),
            AccountMeta::new(to_pub.key(), false),
        ];

        let instructor_dis: u32 = 2;

        let mut intructiondata = Vec::with_capacity(4 + 4);
        intructiondata.extend_from_slice(&instructor_dis.to_le_bytes());
        intructiondata.extend_from_slice(&amount.to_le_bytes());

        //create instuction

        let instruction = Instruction {
            program_id: program_id.key(),
            accounts: accout_meta,
            data: intructiondata,
        };

        invoke(&instruction, &[form_pub, to_pub, program_id])?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct SolTrasfer<'info> {
    #[account(mut)]
    sender: Signer<'info>,

    #[account(mut)]
    recipient: SystemAccount<'info>,

    system_program: Program<'info, System>,
}
