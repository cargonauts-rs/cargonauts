import JSONAPISerializer from 'ember-data/serializers/json-api';

export default JSONAPISerializer.extend({
  keyForAttribute(key) { return key; },

  payloadKeyFromModelName: function(modelName) {
    return modelName;
  }
});
