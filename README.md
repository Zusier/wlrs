![](static/logo.png)
# WLRS [![codecov](https://codecov.io/gh/Zusier/wlrs/branch/main/graph/badge.svg?token=JQX7FH4WL5)](https://codecov.io/gh/Zusier/wlrs)

WLRS (*[/julərs/](http://ipa-reader.xyz/?text=jul%C9%99rs)*) is a WIP workout tracker that is bLaZiNgLy FaSt :rocket: :rocket:.

## TODO

- [x] Make impl for WorkoutEntry so we can parse after retrieving from DB
- [x] Json for data stuff
- [X] figure out HTML + CSS Templating
  - [X] Workout View
  - [X] Status Codes
  - [ ] Workout create
    - Svelte
  - [ ] Homepage
  - [ ] Login/Auth
- [ ] Authentication + Cookies
- [X] switch date to unix timestamp (and start/end times)
- [ ] Switch from sqlite DB once stuff is working great. (Using sqlite because my dev environment changes frequently)

### Optional improvements for future

- [ ] Ability to add custom exercises (Add them to DB instead of in code)

## Design Choices/Questions
- Should I render data server or client side?
  - ~~Most of it will probably be server side, need to research pros and cons~~