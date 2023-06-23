import * as ember from './pkgs/ember.js';

const emberID = BigInt("4612811922963075593");
const unpackedEmberID = ember.decode_ember_id(emberID);

console.log(`Ember ID: ${emberID}`);
console.log(`Ember time: ${ember.get_time(unpackedEmberID).toString()}`);
console.log(`Ember ms since ember epoch (1st May 2023): ${ember.get_epoch_time(unpackedEmberID)}`);
console.log(`Ember node: ${ember.get_node(unpackedEmberID)}`);
console.log(`Ember sequence: ${ember.get_sequence(unpackedEmberID)}`);
console.log(`Ember magic: ${ember.get_magic(unpackedEmberID)}`);

