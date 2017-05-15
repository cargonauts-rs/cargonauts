import DS from 'ember-data';

export default DS.Model.extend({
  text: DS.attr('string'),
  completed: DS.attr('boolean'),
  created_at: DS.attr('date'),
});
