---
nav: Components
group: Application
title: Zencoder
atomId: Zencoder
description: https://zencoder.ai
---

## Icons

```tsx
import { Zencoder } from '@lobehub/icons';
import { Flexbox } from '@lobehub/ui';

export default () => (
  <Flexbox gap={16} horizontal>
    <Zencoder size={64} />
    <Zencoder.Color size={64} />
  </Flexbox>
);
```

## Text

```tsx
import { Zencoder } from '@lobehub/icons';

export default () => <Zencoder.Text size={48} />;
```

## Combine

```tsx
import { Zencoder } from '@lobehub/icons';
import { Flexbox } from '@lobehub/ui';

export default () => (
  <Flexbox gap={16} align={'flex-start'}>
    <Zencoder.Combine size={64} />
    <Zencoder.Combine size={64} type={'color'} />
  </Flexbox>
);
```

## Avatars

```tsx
import { Zencoder } from '@lobehub/icons';
import { Flexbox } from '@lobehub/ui';

export default () => (
  <Flexbox gap={16} horizontal>
    <Zencoder.Avatar size={64} />
    <Zencoder.Avatar size={64} shape={'square'} />
  </Flexbox>
);
```

## Colors

```tsx
import { Zencoder } from '@lobehub/icons';
import { Flexbox } from '@lobehub/ui';

import ColorPreview from '../components/ColorPreview';

export default () => (
  <Flexbox gap={16} horizontal>
    <ColorPreview color={Zencoder.colorPrimary} />
  </Flexbox>
);
```
