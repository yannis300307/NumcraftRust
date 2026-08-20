# How to contribute to Numcraft?
## Steps for creating good issues or pull requests.
If your issue is related to a bug, please give the full error message if one is shown. Please describe the context of the bug, the OS version, the calculator model and if possible, the release version or the latest commit.

If you are asking for an enhancement, please consider the following hardware restrictions:
- No wireless
- No sound
- Numcraft must be compatible with N0110, N0115 and N0120. So make sure that the game runs on every models.
- extremely low amount of ram
- and a lot of other restrictions

Before reporting a bug please read the `Known bugs` section and the `Roadmap` before asking for features.

## About vibe coding and AI
Every AI company has ethical issues: on the crawling side, on the copyright side, on the advertisement side and a lot more.

Numcraft is a human first project. The goal is to show what **human devs** are capable of, not what the latest Fable 19 pro plus extreme can do. Moreover, most of the code that IA produces is not optimized for the calculator or is full of bugs. That's why AI contributions are now forbidden.
However, help from an IA to find a bug in your own code is tolerated as long as you disclose what the AI was used for in your PR.

## Code guideline
- All the code including, function names, comments, structure names, etc... must be in English.
- When a part of your code is not easy to understand or is very abstract, please add comments. Functions like `place_air` that have only a few lines of code don't necessary need comments. 
- Please fit your code to the current architecture of the project. Do not refactor the whole project. If you strongly believe that the code needs a complete refactor, please open a discussion thread first.
- If you use the simulator to write your PR's code, please remember that your code will run on hardware that is thousands of times less powerful than your computer. So always try your code on real hardware before opening a new PR.
