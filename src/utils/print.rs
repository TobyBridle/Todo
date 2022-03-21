pub mod utils
{
    use crate::TodoState;
    use crate::MENU_BAR;
    use ncurses::{addstr, clear, COLOR_PAIR, mv, attron, attroff};
    
    const PADDING: usize = 1;
    
    pub struct PrintController
    {
        pub tab: TodoState,
    }
    
    impl PrintController
    {
        pub fn new() -> PrintController {
            PrintController { tab: TodoState::Other }
        }
        
        pub fn set_state(&mut self, state: TodoState)
        {
            self.tab = state;
        }
        
        pub fn cycle_tab(&mut self)
        {
            let mut states: Vec<TodoState> = Vec::new();
            states.push(TodoState::Other);
            states.push(TodoState::NotDone);
            states.push(TodoState::InProgress);
            states.push(TodoState::Done);
            
            for index in 0..states.len()
            {
                if self.tab == states[index] {
                    self.tab = states[(index + 1) % states.len()];
                    break;
                }
            }
        }
    }

    pub fn print_controls(controller: &PrintController) -> usize
    {
        clear();
        attron(COLOR_PAIR(MENU_BAR));
        addstr("Toggle Todo State (Enter)\tMove Down (J)\tMove Up (K)\tQuit (Q)\tUndo most recent delete (U)");
        mv(1, 0);
        addstr("Add New Todo (A)\t\tRemove Todo (R)\t\t\tSet Todo to In-Progress (D)");
        attroff(COLOR_PAIR(MENU_BAR));
        mv(3, 0);
        if matches!(controller.tab, TodoState::NotDone)
        {
            addstr("ALL\t[TODO]\tIN PROGRESS\tDONE");
        } else if matches!(controller.tab, TodoState::InProgress)
        {
            addstr("ALL\tTODO\t[IN PROGRESS]\tDONE");
        } else if matches!(controller.tab, TodoState::Done)
        {
            addstr("ALL\tTODO\tIN PROGRESS\t[DONE]");        
        } else { addstr("[ALL]\tTODO\tIN PROGRESS\tDONE"); }
        mv(4, 0);
        addstr(&"=".repeat(36));
        
        return (5 + PADDING); // Amount of Lines taken + Padding
    }

}