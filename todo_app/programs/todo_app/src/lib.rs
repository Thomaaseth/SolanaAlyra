use anchor_lang::prelude::*;

declare_id!("GZgYbnUicwDA1N7QZeFygmx86Ncz2V4w83MHVdnxQeMs");

#[program]
pub mod todo_app {
    use super::*;

    pub fn initialize_user(ctx: Context<InitializeUser>, nickname: String) -> Result<()> {
        // Initialiser les attributs de l'utilisateur
        let user = &mut ctx.accounts.user;
        user.todo_count = 0;
        user.user_pubkey = *ctx.accounts.signer.key; // &Pubkey => * => Pubkey
        user.nickname = nickname;

        // user_pubkey la public key du signer
        // todocount = 0
        // nickname = paramètre passé

        Ok(())
    }

    pub fn initialize_todo(ctx: Context<InitializeTodo>, todo_count_index: u32, description: String) -> Result<()> {
        let user = &mut ctx.accounts.user;
        let todo = &mut ctx.accounts.todo;

        // verification que le todo_count_index est bien le user.todo_count
        require_eq!(todo_count_index, user.todo_count + 1, TodoError::InvalidIndex);

        todo.todo_id = todo_count_index;
        todo.status = TodoStatus::Todo;
        todo.description = description;

        user.todo_count += 1;

        Ok(())
    }
    
    pub fn update_todo(ctx: Context<UpdateTodo>, todo_id: u32) -> Result<()> {
        // Get references to the todo and user accounts
        let todo = &mut ctx.accounts.todo;
        
        // Verify the todo_id matches the one in the account
        require_eq!(todo_id, todo.todo_id, TodoError::InvalidIndex);
        
        todo.status = TodoStatus::Done;
        
        Ok(())
    }


    // -> Ajouter une instruction "close_todo" pour supprimer le todo
    // -> Faire les tests unitaires associés (vous avez le initialize_user en exemple)
}

#[derive(Accounts)]
pub struct InitializeUser<'info> {
    // signer => utilisateur qui signe la transaction
    #[account(mut)]
    pub signer: Signer<'info>,
    // user => utilisateur à créer
    #[account(
        init,
        payer = signer,
        space = 8 + User::INIT_SPACE,
        seeds = [b"user", signer.key().as_ref()],
        bump
    )]
    pub user: Account<'info, User>,
    // system_program => alloue l'espace pour l'account à créer
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(todo_count_index: u32)]
pub struct InitializeTodo<'info> {
    // signer
    #[account(mut)]
    pub signer: Signer<'info>,

    // todo
    #[account(
        init,
        payer = signer,
        space = 8 + Todo::INIT_SPACE,
        seeds = [b"todo", signer.key().as_ref(), &todo_count_index.to_le_bytes()],
        bump
    )]
    pub todo: Account<'info, Todo>,

    // user
    #[account(
        mut,
        seeds = [b"user", signer.key().as_ref()],
        bump
    )]
    pub user: Account<'info, User>,

    // system_program
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(todo_id: u32)]
pub struct UpdateTodo<'info> {
    // Signer of the transaction
    pub signer: Signer<'info>,
    
    // The todo that needs to be updated
    #[account(
        mut,
        seeds = [b"todo", signer.key().as_ref(), &todo_id.to_le_bytes()],
        bump
    )]
    pub todo: Account<'info, Todo>,
}

#[account]
#[derive(InitSpace)]
pub struct User {
    user_pubkey: Pubkey,
    #[max_len(30)]
    nickname: String,
    todo_count: u32,
}

#[account]
#[derive(InitSpace)]
pub struct Todo {
    todo_id: u32,
    status: TodoStatus,
    #[max_len(50)]
    description: String,
}

#[derive(InitSpace, AnchorSerialize, AnchorDeserialize, Clone)]
pub enum TodoStatus {
    Todo,
    Done
    // In Progress
}

#[error_code]
pub enum TodoError {
    #[msg("Invalid index")]
    InvalidIndex,
}