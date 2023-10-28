This is in no way affiliated with RobTopGames. This project is purely to help me and others to save their game data in case it doesn't work in the traditional way.


# GD LOCAL SAVE
I'm making this because I can't save my game data the usual way. So I'm making a tool to automate this.

# INSTALLING
To install this, you need to download cargo from `https://rustup.rs` and git from `https://git-scm.com/downloads`
Make sure they're added to PATH.

Then open powershell.
Next step is to navigate to your desired directory by using `cd`.
Then create a new crate by entering the following command:
`cargo new gd_local_save`

Once you created your crate, you can clone this repository by typing: `git clone https://github.com/Nebularzz/gd-local-save`
After that, open your explorer and copy the files in src (from the cloned repository) to the src folder in your crate.
Now navigate to your crate in powershell using `cd` and run `cargo build --release`
Next step is to go to `\target\release` (from the crate in your explorer) and copy `gd_local_save.exe`

The last steps are:
Navigating back to the folder where you created your crate,
Create a new folder,
Paste your executable in there,
Create a new folder in the folder where you pasted the executable and name it `saves` (THIS IS VERY IMPORTANT)

# USAGE
To run this, you need to go to the directory where the executable is and run `./gd_local_save arg` replace arg with either `save` or `load`

# DOCUMENTATION
Currently no documentation.

# ERROR REPORTING
If you run into errors, check if you have done everything correctly in here. But if you still get them please tell me about it.

# DEV NOTE
This is very unstable as of now.
