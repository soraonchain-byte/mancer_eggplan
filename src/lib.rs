use anchor_lang::prelude::*;

// ID Program Anda - Pastikan sinkron dengan Anchor.toml
declare_id!("2wufVp4QcwKpdG9xv2EqaxFYwBzBmTdke8mqVYKRUAv7");

#[program]
pub mod mancer_eggplan {
    use super::*;

    /// Fungsi untuk mendaftarkan kontributor baru dan menentukan Stage awal mereka.
    /// Level 1-10 = Stage 1 (White Egg)
    /// Level 11-20 = Stage 2 (Red Egg), dst.
    pub fn initialize(ctx: Context<Initialize>, starting_level: u64) -> Result<()> {
        // Validasi: Level minimal harus 1
        require!(starting_level > 0, ErrorCode::InvalidLevel);

        let account_data = &mut ctx.accounts.new_account;
        let clock = Clock::get()?;

        // Simpan Data ke Blockchain
        account_data.owner = *ctx.accounts.user.key;
        account_data.level = starting_level;
        account_data.timestamp = clock.unix_timestamp;

        // Logika Penentuan Stage (Setiap 10 Level naik 1 Stage)
        // Level 1-10 / 10 = 0. + 1 = Stage 1
        // Level 11 / 10 = 1. + 1 = Stage 2
        let current_stage = ((starting_level - 1) / 10) + 1;

        msg!("🎋 Mancer Eggplan: Kontributor Baru Berhasil Terdaftar!");
        msg!("Pemilik: {:?}", account_data.owner);
        msg!("Total Level: {}", starting_level);
        msg!("Stage Saat Ini: {}", current_stage);
        msg!("Waktu Registrasi: {}", account_data.timestamp);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    // Inisialisasi account baru dengan alokasi ruang (space)
    // 8 (discriminator) + 32 (pubkey) + 8 (u64 level) + 8 (i64 timestamp)
    #[account(init, payer = user, space = 8 + 32 + 8 + 8)]
    pub new_account: Account<'info, ContributorData>,
    
    #[account(mut)]
    pub user: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

#[account]
pub struct ContributorData {
    pub owner: Pubkey,    // Alamat wallet pemilik
    pub level: u64,      // Level progres (1-50+)
    pub timestamp: i64,  // Waktu saat mendaftar
}

#[error_code]
pub enum ErrorCode {
    #[msg("Level harus dimulai dari angka 1 atau lebih.")]
    InvalidLevel,
}