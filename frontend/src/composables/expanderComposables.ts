import { Component } from 'vue';
import DefaultExpanderEdit from '@/components/hardware/expander/edit/DefaultExpanderEdit.vue';
import PCA9685Edit from '@/components/hardware/expander/edit/PCA9685Edit.vue';
import DefaultExpander from '@/components/hardware/expander/show/DefaultExpander.vue';
import PCA9685Expander from '@/components/hardware/expander/show/PCA9685Expander.vue';

export enum ExpanderType {
  Unknown = '',
  PCA9685 = 'PCA9685',
}

export const useExpanderComponent = (type: keyof typeof ExpanderType): Component | undefined => {
  const mapping = {
    [ExpanderType.Unknown]: DefaultExpander,
    [ExpanderType.PCA9685]: PCA9685Expander,
  };
  return mapping[ExpanderType[type]];
};

export const useExpanderEditComponent = (
  type: keyof typeof ExpanderType,
): Component | undefined => {
  const mapping = {
    [ExpanderType.Unknown]: DefaultExpanderEdit,
    [ExpanderType.PCA9685]: PCA9685Edit,
  };
  return mapping[ExpanderType[type]];
};
