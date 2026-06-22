---
nav: Components
group: Application
title: RooCode
atomId: RooCode
description: https://roocode.com
---

## Icons

```tsx
import { RooCode } from '@lobehub/icons';
import { Flexbox } from '@lobehub/ui';

export default () => <RooCode size={64} />;
```

## Text

```tsx
import { RooCode } from '@lobehub/icons';

export default () => <RooCode.Text size={48} />;
```

## Combine

```tsx
import { RooCode } from '@lobehub/icons';
import { Flexbox } from '@lobehub/ui';

export default () => (
  <Flexbox gap={16} align={'flex-start'}>
    <RooCode.Combine size={64} />
  </Flexbox>
);
```

## Avatars

```tsx
import { RooCode } from '@lobehub/icons';
import { Flexbox } from '@lobehub/ui';

export default () => (
  <Flexbox gap={16} horizontal>
    <RooCode.Avatar size={64} />
    <RooCode.Avatar size={64} shape={'square'} />
  </Flexbox>
);
```

## Colors

```tsx
import { RooCode } from '@lobehub/icons';
import { Flexbox } from '@lobehub/ui';

import ColorPreview from '../components/ColorPreview';

export default () => (
  <Flexbox gap={16} horizontal>
    <ColorPreview color={RooCode.colorPrimary} />
  </Flexbox>
);
```
