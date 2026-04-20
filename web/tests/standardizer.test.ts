import { standardize } from '../web/lib/schema/standardizer';
test('standardizes raw postgres row to Entity', () => {
  const raw = { program_id: 'test-id', name: 'Test' };
  const entity = standardize(raw);
  expect(entity.id).toBe('test-id');
});
