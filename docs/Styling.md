It's important to have consistant and easy to use API for styling.

CSS is the most powerful styling specification ever built, despite this it has horrible ergonomics.

1. What styles work with which widgets.

   In css you can throw any style on any tag, even if nothing changes.
   This can make it difficult to know when styles will actually be applied.

2. The ordering should not matter

   You should be able to chain any method together in any order.

   ```
   rect().radius(5).w(5).gap(5).pad(5)
   //eqiv
   rect().pad(5).w(5).radius(5).gap(5)
   ```

3. Not every widget should be a container/expose the flexbox styles.

   I think that allowing `gap` and `pad` on rect() is a bad choice.

   Every widget should serve a distinct purpose.
