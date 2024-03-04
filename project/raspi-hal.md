---

kanban-plugin: basic

---

## Raspberry Pi HAL initial wrapper/implementation

- [ ] ##### Determine if using `rppal` crate is worth it<br> - Performance advantage over `gpio_cdev`?<br> - Could just use as a basis?<br> - Figure out better pin interrupt interface if do use/improve
- [ ] ##### Determine how much of `gpio_cdev` we want to expose<br> - re-export the crate like `linux-embedded-hal` does?<br> - replicate dynamic-ness or act more like a microcontroller HAL?<br> - event handling?




%% kanban:settings
```
{"kanban-plugin":"basic"}
```
%%