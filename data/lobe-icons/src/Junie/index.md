---
nav: Components
group: Application
title: Junie
atomId: Junie
description: https://jetbrains.com/junie
---

## Icons

```tsx
import { Junie } from '@lobehub/icons';
import { Flexbox } from '@lobehub/ui';

export default () => (
  <Flexbox gap={16} horizontal>
    <Junie size={64} />
    <Junie.Color size={64} />
  </Flexbox>
);
```

## Text

```tsx
import { Junie } from '@lobehub/icons';

export default () => <Junie.Text size={48} />;
```

## Combine

```tsx
import { Junie } from '@lobehub/icons';
import { Flexbox } from '@lobehub/ui';

export default () => (
  <Flexbox gap={16} align={'flex-start'}>
    <Junie.Combine size={64} />
    <Junie.Combine size={64} type={'color'} />
  </Flexbox>
);
```

## Avatars

```tsx
import { Junie } from '@lobehub/icons';
import { Flexbox } from '@lobehub/ui';

export default () => (
  <Flexbox gap={16} horizontal>
    <Junie.Avatar size={64} />
    <Junie.Avatar size={64} shape={'square'} />
  </Flexbox>
);
```

## Colors

```tsx
import { Junie } from '@lobehub/icons';
import { Flexbox } from '@lobehub/ui';

import ColorPreview from '../components/ColorPreview';

export default () => (
  <Flexbox gap={16} horizontal>
    <ColorPreview color={Junie.colorPrimary} />
  </Flexbox>
);
```
