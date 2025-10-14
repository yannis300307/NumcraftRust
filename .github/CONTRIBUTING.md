# How to contribute to Numcraft?
## Steps for creating good issues or pull requests.
If your issue is related to a bug, please give the full error message if one is shown. Please describe the context of the bug, the OS version, the calculator model and if possible, the release version or the latest commit.

If you are asking for an enhancement, please concider the hardware resptrictions :
- Multiplayer is currently impossible
- No sound
- Numcraft must be compatible with N0110, N0115 and N0120. So make sure that the game runs on N0110 or N0115.
- and a lot of other restrictions
- extremelly low amount of ram

Before reporting a bug please read the `Known bugs` section and the `Roadmap` before asking for features.

## About vibe codding and AI
Nowdays, vibe codding is more and more used. It's an easy way to write code quickly but AIs have a big problem when it comes to programming on a calculator: they don't know the hardware. This is mainly due to the lack of documentation about this hardware. Moreover, I prefer code that is written by humans. For me, code is an art (and particularly video games) so I encourage handwriting code.

So please, avoid vibe codding if you can. If you still want to use AI to write your code please:
 - Limit the work of the AI to a single function or a few ones.
 - Don't let the AI architecturing the project.
 - Don't let the AI modifiying existing code when it's not necessary.
 - Don't let the AI writing very low level code.
 - Carrefully read the generated code. Do not open a pull request if it works, read the entiere generated code before openning it. And be extremelly careful about the rendering.

## Code guideline
- All the code including, function names, commentaries, structures names, etc... must be in english.
- When a part of your code is not easy to understand or is very abstract, please add comments. Functions like `place_air` and that have only a few lines of code doesn't necessary need comments. 
- Please fit your code to the current architecture of the project. Do not refactor the whole project. If you strongly believe that the code needs a complete refactor, please open a discussion thread first.
- If you use the simulator to code your PR, please remember that your code will run on machines that are thousands of time less powerful than your computer. So always try your code on real hardware before openning a new PR.
