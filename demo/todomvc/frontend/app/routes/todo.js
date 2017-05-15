import Ember from 'ember';

export default Ember.Route.extend({
  model() {
    return this.store.findAll('note').then(notes => notes.sortBy("created_at"));
  },

  actions: {
    createNote(text) {
      var self = this;

      this.store.createRecord('note', {
        text: text,
        completed: false
      }).save().then(function() { self.refresh(); });
    },

    destroyNote(id) {
      var self = this;

      this.store.findRecord('note', id)
          .then(note => note.destroyRecord())
          .then(function() { self.refresh(); })
    },

    completeNote(id) {
      var self = this;

      this.store.findRecord('note', id).then(function(note) {
        note.set('completed', true);
        note.save().then(function() { self.refresh(); });
      })
    },

    clearCompleted() {
      var self = this;

      this.store.findAll('note')
          .then(notes => notes.filterBy('completed'))
          .then(notes => notes.each(note => note.destroyRecord()))
          .then(function() { self.refresh(); });
    }
  }
});
