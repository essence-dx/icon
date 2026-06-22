import React, { memo } from 'react';
import { Path, Svg } from 'react-native-svg';

import type { RNIconProps } from '@/features';

const Icon = memo<RNIconProps>(({ size = 24, style, color = '#000000', ...rest }) => {
  return (
    <Svg
      color={color}
      fillRule="evenodd"
      height={size}
      style={style}
      viewBox="0 0 24 24"
      width={size}
      {...rest}
    >
      <Path
        d="M24 9.333C24 18.666 20 24 9.333 24H8v-8h1.333C14 16 16 14 16 9.333V8h8v1.333zM8 16H0V8h8v8zM16 8H8V0h8v8z"
        fill={color}
      />
    </Svg>
  );
});

Icon.displayName = 'JunieMono';

export default Icon;
