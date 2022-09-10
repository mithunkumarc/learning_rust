#### Associated methods: 

      Methods without a self type as their first parameter. 
      It is similar to a static method in object-oriented languages.
      These methods are available on the type themselves and do not need an instance
      of the type to invoke them. Associated methods are invoked by prefixing the
      method name with the struct name and double colons, like so:

        Player::with_name("Dave");

#### Instance methods: 

      Functions that take a self value as its first argument. The
      self symbol here is similar to self in Python and points to the instance on which
      the method is implemented. Instance require to call these methods.

      let player = Player::with_name("Dave");
      player.get_friends(
